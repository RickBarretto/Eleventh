use crate::Request;
use crate::Response;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct Client {
    url: String,
}

impl Client {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn get(&self, path: &str) -> Response {
        // Ensure we pass a path starting with '/' so the server's URL parser
        // can correctly interpret it (Request::parse_url expects a full_path
        // like "/some/path?query=1"). The stored `url` is used only for the
        // TCP connection (host:port).
        let path = format!("/{}", path.trim_start_matches('/'));
        let req = Request::new("GET", &path, "");
        self.request(&req)
    }

    pub fn post(&self, path: &str, body: &str) -> Response {
        let path = format!("/{}", path.trim_start_matches('/'));
        let req = Request::new("POST", &path, body);
        self.request(&req)
    }

    pub fn delete(&self, path: &str) -> Response {
        let path = format!("/{}", path.trim_start_matches('/'));
        let req = Request::new("DELETE", &path, "");
        self.request(&req)
    }

    fn request(&self, req: &Request) -> Response {
        let mut stream = TcpStream::connect(self.url()).expect("Failed to connect to server");
        let raw_request: String = req.to_string();

        stream
            .write_all(raw_request.as_bytes())
            .expect("Failed to send request");

        let mut buffer = Vec::new();
        stream
            .read_to_end(&mut buffer)
            .expect("Failed to read response");

        let raw_response = String::from_utf8_lossy(&buffer);
        let response = Response::from_raw(&raw_response).expect("Failed to parse response");
        response
    }
}
