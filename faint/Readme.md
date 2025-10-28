# Faint!

```sh
$ deno run faint http://127.0.0.1:8080/match/1/start/ -x POST --requets 80000 --concurrency 1000
```

## Real Example and Numbers

```sh
$ deno run faint
Task faint deno run --allow-net main.ts

Usage: deno run --allow-net script.ts [options] <url>

Options:
  -h, --help            Show this help message
  -r, --requests <n>    Number of total requests (default: 80000)
  -c, --concurrency <n> Number of concurrent requests (default: 5000)
  -x, --method <m>      HTTP method to use (default: GET)

Examples:
  deno run --allow-net script.ts -r 10000 -c 1000 -x POST http://localhost:8080

$ deno run faint http://127.0.0.1:8080/match/1/start/ -x POST --requets 80000 --concurrency 1000
Task faint deno run --allow-net main.ts "http://127.0.0.1:8080/match/1/start/" "-x" "POST" "--requets" "80000" "--concurrency" "1000"
Accepted: 79925
Rejected: 75
Total requests: 80000
Concurrency limit: 1000
Time taken: 83.95 s
Throughput: 952.92 req/s

$ deno run main.ts http://127.0.0.1:8080/match/1/start/ -x POST --requets 80000 --concurrency 1000 
✅ Granted net access to "127.0.0.1:8080".
Accepted: 79927
Rejected: 73
Total requests: 80000
Concurrency limit: 1000
Time taken: 81.13 s
Throughput: 986.08 req/s

$ deno run main.ts http://127.0.0.1:8080/match/1/start/ -x POST --requets 80000 --concurrency 10000
✅ Granted net access to "127.0.0.1:8080".
Accepted: 79993
Rejected: 7
Total requests: 80000
Concurrency limit: 10000
Time taken: 117.27 s
Throughput: 682.21 req/s

$ deno run main.ts http://127.0.0.1:8080/match/1/start/ -x POST --requets 80000 --concurrency 40000
✅ Granted net access to "127.0.0.1:8080".
Accepted: 80000
Rejected: 0
Total requests: 80000
Concurrency limit: 40000
Time taken: 201.79 s
Throughput: 396.45 req/s
```
