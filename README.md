# Eleventh

<p align="center">Only 11 wins.</p>

**Eleventh** is a turn-based card game inspired by FIFA’s Ultimate Team cards and the strategic style of Soccer Manager.
Players build their own dream team using collectible cards, manage tactics, and compete in tactical duels against other managers. 


## Running the system


### Starting the server

```sh
$ cargo run -p server
```

### Starting the client

```sh
$ cargo run -p eleventh
```

### Running tests

```sh
$ cargo test
```

## Manually testing the system

If you want to know if the system works well with external tools, you may use `curl` for any HTTP endpoint declared.
Remember, if you need to change the method, use `-X <METHOD>` and a body, use `-d <BODY>`.

### Account Creation Example

```sh
$ curl http://127.0.0.1:8080/accounts/
$ curl -X POST http://127.0.0.1:8080/accounts/create/ -d '{"username": "Rick", "password": "123456"}'
$ curl -X POST http://127.0.0.1:8080/accounts/login/ -d '{"username": "Rick", "password": "123456"}'
```