import { settingsFromCLI, logResults } from "./cli.ts"

let accepted = 0
let rejected = 0

async function request(url: string, method: string) {
    try {
        (await fetch(url, { method }))? accepted++ : rejected++
    } catch {
        rejected++
    }
}

type Settings = {
    url: string,
    method: string,
    requests: number,
    concurrency: number,
}

type Report = {
    accepted: number,
    rejected: number,
    concurrency: number,
    miliseconds: number,
}

async function execute({requests, concurrency, url, method}: Settings): Promise<Report> {
    const start = performance.now()

    const tasks: Promise<void>[] = []
    for (let i = 0; i < requests; i++) {
        tasks.push(request(url, method))

        if (tasks.length >= concurrency) {
            await Promise.all(tasks)
            tasks.length = 0
        }
    }

    await Promise.all(tasks)

    return {
        accepted,
        rejected,
        concurrency,
        miliseconds: performance.now() - start
    }    
}

async function main() {
    const settings = settingsFromCLI(Deno.args)
    const report = await execute(settings)
    logResults(report)
}

main()
