import {parseArgs} from "@std/cli"
import { ms_to_s } from "./utils.ts"

export function settingsFromCLI(cliArgs: string[]): CliSettings {
    const parsed = parseArgs(cliArgs, {
        boolean: ["help"],
        string: ["requests", "concurrency", "method"],
        alias: {
            "help": "h",
            "requests": "r",
            "concurrency": "c",
            "method": "x"
        },
        default: {
            "help": false,
            "requests": 80000,
            "concurrency": 5000,
            "method": "GET"
        }
    })

    if (parsed.help) {
        console.log(help())
        Deno.exit()
    }

    if (Deno.args.length < 1) {
        console.error(help())
        Deno.exit(1)
    }

    const url: string = Deno.args[0] ?? ""
    const requests: number = parseInt(parsed.requests as string, 10)
    const concurrency: number = parseInt(parsed.concurrency as string, 10)
    const method: string = parsed.method.toUpperCase()

    return {url, requests, concurrency, method }
}

export function logResults({accepted, rejected, concurrency, miliseconds}: Report): void {
    const seconds = ms_to_s(miliseconds)
    const total: number = accepted + rejected
    const throughput: number = total / seconds

    console.log(`Accepted: ${accepted}`)
    console.log(`Rejected: ${rejected}`)
    console.log(`Total requests: ${total}`)
    console.log(`Concurrency limit: ${concurrency}`)
    console.log(`Time taken: ${seconds.toFixed(2)} s`)
    console.log(`Throughput: ${throughput.toFixed(2)} req/s`)
}

// =~=~=~=~=~=~ Internal ~=~=~=~=~=~=

type CliSettings = {
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

function help() {
    return `
Usage: deno run faint [options] <url>

Options:
  -h, --help            Show this help message
  -r, --requests <n>    Number of total requests (default: 80000)
  -c, --concurrency <n> Number of concurrent requests (default: 5000)
  -x, --method <m>      HTTP method to use (default: GET)

Examples:
  deno run --allow-net script.ts -r 10000 -c 1000 -x POST http://localhost:8080
`
}