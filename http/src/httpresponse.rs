use std::collections::HashMap;
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> HttpResponse<'a> {
    pub fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }

    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response = HttpResponse::default();

        // Override status code if different from default
        if status_code != "200" {
            response.status_code = status_code;
        }

        // Determine status text
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Unknown",
        };

        // Handle headers
        response.headers = match headers {
            Some(h) => Some(h),
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        // Set body
        response.body = body;

        response
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn response_default() {
        let response: HttpResponse = HttpResponse::default();
        assert_eq!("HTTP/1.1", response.version);
        assert_eq!("200", response.status_code);
        assert_eq!("OK", response.status_text);
        assert_eq!(None, response.headers);
        assert_eq!(None, response.body);
    }

    #[test]
    fn test_response_new() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "application/json");

        let body_text = Some("{\"message\":\"Bad request\"}".to_string());

        let response = HttpResponse::new("400", Some(headers), body_text.clone());

        assert_eq!("HTTP/1.1", response.version);
        assert_eq!("400", response.status_code);
        assert_eq!("Bad Request", response.status_text);
        assert_eq!(
            Some("application/json"),
            response
                .headers
                .as_ref()
                .unwrap()
                .get("Content-Type")
                .copied()
        );
        assert_eq!(body_text, response.body);
    }
}
