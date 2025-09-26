/**
 * Simple concurrency test: spawn many parallel claims against a server endpoint
 * Usage: deno run --allow-net scripts/claim_test.ts http://localhost:8000
 */
const [url] = Deno.args;
if (!url) {
  console.error("Usage: claim_test.ts <server_url>");
  Deno.exit(1);
}

const fetchClaim = async (userId: string) => {
    const res = await fetch(`${url}/claim`, {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ userId }),
    })

    return res.status === 200 ? await res.json() 
        : { status: res.status, body: await res.text() }
};

const main = async () => {
    const userCount = 100;
    const users: string[] = [];
    for (let i = 0; i < userCount; i++) {
        const r = await fetch(`${url}/join`);
        const j = await r.json();
        users.push(j.id);
    }

    console.log(`Created ${users.length} users`)

    const promises = users.map((u) => fetchClaim(u))
    const results = await Promise.all(promises)

    const successes = results.filter((r) => r?.pack).length
    console.log(`Successes: ${successes}`)
    console.log(`Results sample:`, results.slice(0, 5))
};

main();
