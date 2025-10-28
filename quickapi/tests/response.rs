#[cfg(test)]
extern crate speculate;
#[cfg(test)]
use speculate::speculate;

use serde_json::json;
use quickapi::response::Response;



speculate! {
    describe "Building Response" {
        it "may be a plain text" {
            let resp = Response::ok().plain("hello");
            assert_eq!(resp.status, 200);
            assert_eq!(resp.reason, "OK");
            assert_eq!(resp.body, "hello");
            assert_eq!(resp.content_type, "text/plain");
        }

        it "may be a html" {
            let resp = Response::ok().html("<h1>hi</h1>");
            assert_eq!(resp.status, 200);
            assert_eq!(resp.reason, "OK");
            assert_eq!(resp.body, "<h1>hi</h1>");
            assert_eq!(resp.content_type, "text/html");
        }

        it "may be a xml" {
            let resp = Response::ok().xml("<tag>value</tag>");
            assert_eq!(resp.status, 200);
            assert_eq!(resp.reason, "OK");
            assert_eq!(resp.body, "<tag>value</tag>");
            assert_eq!(resp.content_type, "application/xml");
        }

        it "may be a  json" {
            let value = json!({"key": "value"});
            let resp = Response::ok().json(&value);
            assert_eq!(resp.status, 200);
            assert_eq!(resp.reason, "OK");
            assert_eq!(resp.body, value.to_string());
            assert_eq!(resp.content_type, "application/json");
        }

        it "may be any content" {
            let resp = Response::ok().content("data", "application/octet-stream");
            assert_eq!(resp.status, 200);
            assert_eq!(resp.reason, "OK");
            assert_eq!(resp.body, "data");
            assert_eq!(resp.content_type, "application/octet-stream");
        }
    }

    describe "A Response" {
        it "supports to_string and from_raw" {
            let resp = Response::ok().plain("body");
            let raw = resp.to_string();
            let parsed = Response::from_raw(&raw).unwrap();
            assert_eq!(parsed.status, 200);
            assert_eq!(parsed.reason, "OK");
            assert_eq!(parsed.body, "body");
            assert_eq!(parsed.content_type, "text/plain");
        }

        it "from_raw is None if invalid length" {
            let raw = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 10\r\n\r\nshort";
            assert!(Response::from_raw(raw).is_none());
        }

        it "has custom status" {
            let resp = Response::custom(418, "I'm a teapot").plain("short and stout");
            assert_eq!(resp.status, 418);
            assert_eq!(resp.reason, "I'm a teapot");
            assert_eq!(resp.body, "short and stout");
            assert_eq!(resp.content_type, "text/plain");
        }

        it "has OK status" {
            let resp = Response::ok().build();
            assert_eq!(resp.status, 200);
            assert_eq!(resp.reason, "OK");
            assert_eq!(resp.body, "");
        }

        it "has bad request status" {
            let resp = Response::bad_request().build();
            assert_eq!(resp.status, 400);
            assert_eq!(resp.reason, "Bad Request");
            assert_eq!(resp.body, "400 Bad Request");
        }

        it "has unauthorized status" {
            let resp = Response::unauthorized().build();
            assert_eq!(resp.status, 401);
            assert_eq!(resp.reason, "Unauthorized");
            assert_eq!(resp.body, "401 Unauthorized");
        }

        it "has not found status" {
            let resp = Response::not_found().build();
            assert_eq!(resp.status, 404);
            assert_eq!(resp.reason, "Not Found");
            assert_eq!(resp.body, "404 Not Found");
        }

        it "has internal error status" {
            let resp = Response::internal_error().build();
            assert_eq!(resp.status, 500);
            assert_eq!(resp.reason, "Internal Server Error");
            assert_eq!(resp.body, "500 Internal Server Error");
        }

        it "implements into string" {
            let resp = Response::ok().plain("abc");
            let s: String = resp.into();
            assert!(s.contains("HTTP/1.1 200 OK"));
            assert!(s.contains("Content-Type: text/plain"));
            assert!(s.contains("Content-Length: 3"));
            assert!(s.ends_with("abc"));
        }
    }
}