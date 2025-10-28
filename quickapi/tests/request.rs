#[cfg(test)]
extern crate speculate;
use quickapi::Request;
#[cfg(test)]
use speculate::speculate;


speculate! {

    describe "Some Request" {

        it "creates a new request" {
            let req = Request::new("GET", "/api/data?foo=bar&baz=qux", "body");
            assert_eq!(req.method, "GET");
            assert_eq!(req.path, "/api/data");
            assert_eq!(req.body, "body");
            assert_eq!(req.query.get("foo"), Some(&"bar".to_string()));
            assert_eq!(req.query.get("baz"), Some(&"qux".to_string()));
        }

        it "parses from raw" {
            let raw = "POST /submit?x=1&y=2 HTTP/1.1\r\nHost: localhost\r\n\r\npayload";
            let req = Request::from_raw(raw).unwrap();
            assert_eq!(req.method, "POST");
            assert_eq!(req.path, "/submit");
            assert_eq!(req.body, "payload");
            assert_eq!(req.query.get("x"), Some(&"1".to_string()));
            assert_eq!(req.query.get("y"), Some(&"2".to_string()));
        }

        it "gets param" {
            let req = Request::new("GET", "/test?a=42", "");
            assert_eq!(req.param("a"), Some(&"42".to_string()));
            assert_eq!(req.param("missing"), None);
        }

        it "displays request" {
            let req = Request::new("GET", "/hello?name=world", "hi");
            let s = format!("{}", req);
            assert!(s.contains("GET /hello?name=world HTTP/1.1"));
            assert!(s.contains("Params:"));
            assert!(s.contains("hi"));
        }

        it "handles empty request" {
            let err = Request::from_raw("").unwrap_err();
            assert_eq!(err, "Empty request");
        }
    }
}
