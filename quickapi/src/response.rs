pub struct Response {
    pub status: u16,
    pub reason: String,
    pub body: String,
    pub content_type: String,
}

pub struct ResponseBuilder {
    status: u16,
    reason: String,
    body: String,
    content_type: String,
}

fn resp_to_string(status: u16, reason: &str, content_type: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        reason,
        content_type,
        body.len(),
        body
    )
}

impl ResponseBuilder {
    pub fn plain(mut self, body: &str) -> Response {
        self.body = body.to_string();
        self.content_type = "text/plain".to_string();
        self.build()
    }

    pub fn html(mut self, body: &str) -> Response {
        self.body = body.to_string();
        self.content_type = "text/html".to_string();
        self.build()
    }

    pub fn xml(mut self, body: &str) -> Response {
        self.body = body.to_string();
        self.content_type = "application/xml".to_string();
        self.build()
    }

    pub fn json(mut self, body: &serde_json::Value) -> Response {
        self.body = body.to_string();
        self.content_type = "application/json".to_string();
        self.build()
    }

    pub fn content(mut self, body: &str, content_type: &str) -> Response {
        self.body = body.to_string();
        self.content_type = content_type.to_string();
        self.build()
    }

    pub fn build(self) -> Response {
        Response {
            status: self.status,
            reason: self.reason,
            body: self.body,
            content_type: self.content_type,
        }
    }
}

impl Response {

    pub fn from_raw(raw: &str) -> Option<Response> {
        let (mut lines, status, reason) = parse_header(raw)?;

        let mut content_type = "text/plain".to_string();
        let mut content_length = 0;

        for line in &mut lines {
            if line.is_empty() { break; }
            parse_meta(&mut content_type, &mut content_length, line)?;
        }

        let body: String = lines.collect::<Vec<&str>>().join("\n");
        if body.len() != content_length { return None; }
        Some(Response {
            status,
            reason,
            body,
            content_type,
        })
    }

    pub fn custom(status: u16, reason: &str) -> ResponseBuilder {
        ResponseBuilder {
            status,
            reason: reason.into(),
            body: String::new(),
            content_type: "text/plain".into(),
        }
    }

    pub fn ok() -> ResponseBuilder {
        ResponseBuilder {
            status: 200,
            reason: "OK".into(),
            body: String::new(),
            content_type: "text/plain".into(),
        }
    }

    pub fn bad_request() -> ResponseBuilder {
        ResponseBuilder {
            status: 400,
            reason: "Bad Request".into(),
            body: "400 Bad Request".into(),
            content_type: "text/plain".into(),
        }
    }

    pub fn unauthorized() -> ResponseBuilder {
        ResponseBuilder {
            status: 401,
            reason: "Unauthorized".into(),
            body: "401 Unauthorized".into(),
            content_type: "text/plain".into(),
        }
    }

    pub fn not_found() -> ResponseBuilder {
        ResponseBuilder {
            status: 404,
            reason: "Not Found".into(),
            body: "404 Not Found".into(),
            content_type: "text/plain".into(),
        }
    }

    pub fn internal_error() -> ResponseBuilder {
        ResponseBuilder {
            status: 500,
            reason: "Internal Server Error".into(),
            body: "500 Internal Server Error".into(),
            content_type: "text/plain".into(),
        }
    }
}

fn parse_meta(content_type: &mut String, content_length: &mut usize, line: &str) -> Option<()> {
    let mut header_parts = line.splitn(2, ':');
    let header_name = header_parts.next()?.trim();
    let header_value = header_parts.next()?.trim();
    Some(match header_name.to_lowercase().as_str() {
        "content-type" => *content_type = header_value.to_string(),
        "content-length" => *content_length = header_value.parse().ok()?,
        _ => {}
    })
}

fn parse_header(raw: &str) -> Option<(std::str::Lines<'_>, u16, String)> {
    let mut lines = raw.lines();
    let status_line = lines.next()?;
    let mut status_parts = status_line.splitn(3, ' ');
    let _http_version = status_parts.next()?;
    let status = status_parts.next()?.parse().ok()?;
    let reason = status_parts.next()?.to_string();
    Some((lines, status, reason))
}

impl ToString for Response {
    fn to_string(&self) -> String {
        resp_to_string(self.status, &self.reason, &self.content_type, &self.body)
    }
}

impl Into<String> for Response {
    fn into(self) -> String {
        self.to_string()
    }
}
