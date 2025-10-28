# QuickAPI

## Usage

```rs
let app: Server = Server::new();

// /greet?name="Rick" => Hello, Rick?
app.route("GET", "/greet", |req, _params| async move {
    let binding: String = "Anonymous".to_string();
    let name: &String = req.param("name").unwrap_or(&binding);

    Response::ok().plain(&format!("Hello, {}!", name))
});

// /users/123 => {"user_id":"123"}
app.route("GET", "/users/{id}", |_req, params| async move {
    let id: &String = params.get("id").unwrap();

    Response::ok().json(&json!({"user_id": id}))
});

app.run("127.0.0.1:8080").await;
```
