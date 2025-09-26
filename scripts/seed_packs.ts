import { connect } from "https://deno.land/x/redis@v0.29.4/mod.ts"
import { v4 } from "https://deno.land/std@0.203.0/uuid/mod.ts"

const REDIS_HOST = Deno.env.get("REDIS_HOST") ?? "127.0.0.1"
const REDIS_PORT = Number(Deno.env.get("REDIS_PORT") ?? 6379)

const redis = await connect({ hostname: REDIS_HOST, port: REDIS_PORT })

const PACK_POOL_KEY = "packs:pool"
const PACK_DATA_KEY = "packs:data"

console.log("Seeding packs...")
const packs = [];
for (let i = 0; i < 200; i++) {
    const id = v4.generate()
    const pack = {
        id,
        theme: "soccer",
        cards: [
        { id: `${id}-c1`, name: "Forward", rating: Math.ceil(Math.random() * 5) },
        { id: `${id}-c2`, name: "Midfielder", rating: Math.ceil(Math.random() * 5) },
        ],
    };
    packs.push(pack)
}

for (const pack of packs) {
    await redis.rpush(PACK_POOL_KEY, pack.id)
    await redis.hset(PACK_DATA_KEY, pack.id, JSON.stringify(pack))
}

console.log(`Seeded ${packs.length} packs`)
await redis.close()
