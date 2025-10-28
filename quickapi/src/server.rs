use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use regex::Regex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub use crate::request::Request;
pub use crate::response::Response;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
    Other(String),
}

impl HttpMethod {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            "OPTIONS" => HttpMethod::OPTIONS,
            "HEAD" => HttpMethod::HEAD,
            other => HttpMethod::Other(other.to_string()),
        }
    }
}

pub type BoxedFuture = Pin<Box<dyn Future<Output = Response> + Send>>;
pub type RouteAction = Arc<dyn Fn(Request, HashMap<String, String>) -> BoxedFuture + Send + Sync>;

pub struct Route {
    pub method: HttpMethod,
    pub pattern: Regex,
    pub param_names: Vec<String>,
    pub handler: RouteAction,
}

impl Clone for Route {
    fn clone(&self) -> Self {
        Self {
            method: self.method.clone(),
            pattern: self.pattern.clone(),
            param_names: self.param_names.clone(),
            handler: Arc::clone(&self.handler),
        }
    }
}

pub struct Server<T> {
    pub services: Arc<T>,
    routes: Vec<Route>,
}

impl<T> Server<T> {
    pub fn new(services: T) -> Self {
        Self {
            routes: Vec::new(),
            services: Arc::new(services),
        }
    }

    pub fn route<F, Fut>(&mut self, method: &str, path: &str, handler: F)
    where
        F: Fn(Request, HashMap<String, String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        let method = HttpMethod::from_str(method);
        let mut param_names = Vec::new();
        let mut regex_str = regex::escape(path)
            .replace(r"\{", "{")
            .replace(r"\}", "}")
            .replace("{", "(?P<")
            .replace("}", ">[^/]+)")
            .replace(r"\?", "?");

        if regex_str.ends_with('/') {
            regex_str = format!("{}?", &regex_str);
        } else {
            regex_str = format!("{}/?", &regex_str);
        }

        let regex = Regex::new(&format!("^{}$", regex_str)).unwrap();

        for cap in Regex::new(r"\(\?P<([^>]+)>[^)]+\)")
            .unwrap()
            .captures_iter(&regex_str)
        {
            param_names.push(cap[1].to_string());
        }

        let route = Route {
            method,
            pattern: regex,
            param_names,
            handler: Arc::new(move |req, params| Box::pin(handler(req, params))),
        };
        self.routes.push(route);
    }

    pub fn get<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request, HashMap<String, String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.route("GET", path, handler);
    }

    pub fn post<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request, HashMap<String, String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.route("POST", path, handler);
    }

    pub fn put<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request, HashMap<String, String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.route("PUT", path, handler);
    }

    pub fn delete<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request, HashMap<String, String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.route("DELETE", path, handler);
    }

    pub fn patch<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request, HashMap<String, String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.route("PATCH", path, handler);
    }

    pub fn options<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request, HashMap<String, String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.route("OPTIONS", path, handler);
    }

    pub fn head<F, Fut>(&mut self, path: &str, handler: F)
    where
        F: Fn(Request, HashMap<String, String>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Response> + Send + 'static,
    {
        self.route("HEAD", path, handler);
    }

    pub async fn simulate(&self, method: &str, path: &str, body: &str) -> Response {
        let request = Request::new(method, path, body);

        for route in &self.routes {
            if route.method == HttpMethod::from_str(&request.method)
                && route.pattern.captures(&request.path).is_some()
            {
                let params = parameters(&request, route);
                return (route.handler)(request, params).await;
            }
        }

        Response::not_found().build()
    }

    pub async fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr)
            .await
            .expect("Failed to bind TCP listener");

        println!("Listening on {}", addr);

        loop {
            match listener.accept().await {
                Ok((socket, _)) => {
                    let routes = self.routes.clone();
                    tokio::spawn(async move {
                        handle_connection(socket, routes).await;
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}. Skipped!", e);
                    continue;
                }
            }
        }
    }
}

async fn handle_connection(mut socket: TcpStream, routes: Vec<Route>) {
    let mut buf: Vec<u8> = vec![0; 1024];

    let n: usize = match socket.read(&mut buf).await {
        Ok(n) => n,
        Err(_) => {
            let resp = Response::bad_request().build();
            let _ = socket.write_all(resp.to_string().as_bytes()).await;
            return;
        }
    };

    let request_raw = String::from_utf8_lossy(&buf[..n]);

    let req = match Request::from_raw(&request_raw) {
        Ok(req) => req,
        Err(_) => {
            let resp = Response::bad_request().build();
            let _ = socket.write_all(resp.to_string().as_bytes()).await;
            return;
        }
    };

    println!(">>>>>>>");
    println!("{}", req);
    println!("<<<<<<<");

    for route in &routes {
        if route.method == HttpMethod::from_str(&req.method)
            && route.pattern.captures(&req.path).is_some()
        {
            let params = parameters(&req, route);
            let response: Response = (route.handler)(req, params).await;
            let resp_text: String = response.to_string();
            println!("{}", &resp_text);
            let _ = socket.write_all(resp_text.as_bytes()).await;
            return;
        }
    }

    let resp: Response = Response::not_found().build();
    println!("{}", &resp.to_string());
    let _ = socket.write_all(resp.to_string().as_bytes()).await;
}

fn parameters(request: &Request, route: &Route) -> HashMap<String, String> {
    let captures = route.pattern.captures(&request.path).unwrap();
    let mut parameters: HashMap<String, String> = HashMap::new();

    for name in &route.param_names {
        if let Some(m) = captures.name(name) {
            parameters.insert(name.clone(), m.as_str().to_string());
        }
    }

    parameters
}
