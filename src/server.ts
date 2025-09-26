import { Application, Router, Context } from "https://deno.land/x/oak@v12.6.0/mod.ts";
import { connect } from "https://deno.land/x/redis@v0.29.4/mod.ts";
import { v4 } from "https://deno.land/std@0.203.0/uuid/mod.ts";

const REDIS_HOST = Deno.env.get("REDIS_HOST") ?? "redis";
const REDIS_PORT = Number(Deno.env.get("REDIS_PORT") ?? 6379);

const redis = await connect({ hostname: REDIS_HOST, port: REDIS_PORT });

// Keys used in Redis
const PACK_POOL_KEY = "packs:pool"; // a list of pack IDs
const PACK_DATA_KEY = "packs:data"; // hash packId -> JSON
const PACK_OWNER_KEY = "packs:owner"; // hash packId -> userId
const USER_KEY_PREFIX = "user:"; // hash user:<id> -> {id, last_claim_ts}

const app = new Application()
const router = new Router()

const userKeyOf = (id: string) => USER_KEY_PREFIX + id

router.get("/join", async (context: Context) => {
    const id = v4.generate()
    const key = userKeyOf(id)
    const now = Date.now()
    await redis.hset(key, { id, last_claim_ts: "0" })

    context.response.status = 200
    context.response.body = { id }
});

router.post("/claim", async (context: Context) => {
    const body = await context.request
        .body({ type: "json" })
        .value
        .catch(() => ({}))

    const userId = body?.userId

    if (!userId) {
        context.response.status = 400
        context.response.body = { error: "userId required" }
        return
    }

    if (!await redis.exists(userKeyOf(userId))) {
        context.response.status = 404
        context.response.body = { error: "user not found" }
        return
    }

    const now = Date.now()
    const lastClaim = Number(await redis.hget(userKeyOf(userId), "last_claim_ts") ?? 0)
    const DAY = 24 * 60 * 60 * 1000

    if (now - lastClaim < DAY) {
        context.response.status = 429
        context.response.body = { error: "claim available once every 24h" }
        return
    }

    // Atomic pop from list
    const packId = await redis.lpop(PACK_POOL_KEY);
    if (!packId) {
        context.response.status = 410;
        context.response.body = { error: "no packs available" };
        return;
    }

    // record ownership
    await redis.hset(PACK_OWNER_KEY, packId, userId);
    await redis.hset(userKeyOf(userId), { last_claim_ts: String(now) });

    const packJson = await redis.hget(PACK_DATA_KEY, packId);
    const pack = packJson ? JSON.parse(packJson) : { id: packId, cards: [] };

    context.response.status = 200;
    context.response.body = { pack };
});

router.post("/match", async (context: Context) => {
    // Simplified match: body = { hostId, guestId }
    const body = await context.request
        .body({ type: "json" })
        .value
        .catch(() => ({}))
    
    const hostId = body?.hostId
    const guestId = body?.guestId

    if (!hostId || !guestId) {
        context.response.status = 400
        context.response.body = { error: "hostId and guestId required" }
        return
    }

    // For week1: host always wins 1x0
    const result = { 
        host: hostId, 
        guest: guestId, 
        score: "1:0", 
        winner: hostId 
    }

    // Publish result so other servers can relay to clients (simple Redis channel)
    const channel = `match:results`
    await redis.publish(channel, JSON.stringify(result))

    context.response.status = 200
    context.response.body = { result }
})

router.post("/trade", async (context: Context) => {
    // trade: { fromUser, toUser, packId }
    const body = await context.request
        .body({ type: "json" })
        .value
        .catch(() => ({}))

    const from = body?.fromUser
    const to = body?.toUser
    const packId = body?.packId

    if (!from || !to || !packId) {
        context.response.status = 400
        context.response.body = { error: "fromUser,toUser,packId required" }
        return
    }

    const owner = await redis.hget(PACK_OWNER_KEY, packId);
    if (owner !== from) {
        context.response.status = 403;
        context.response.body = { error: "not the owner" };
        return;
    }

    await redis.hset(PACK_OWNER_KEY, packId, to)
    context.response.status = 200
    context.response.body = { ok: true }
})


// server health check
router.get("/health", (context) => {
    context.response.body = { ok: true }
})


app.use(router.routes())
app.use(router.allowedMethods())

const PORT = Number(Deno.env.get("PORT") ?? 8000)
console.log(`Server listening on :${PORT}`)
await app.listen({ port: PORT })
