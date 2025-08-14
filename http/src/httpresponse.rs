use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> From<&HttpResponse<'a>> for String {
    fn from(res: &HttpResponse<'a>) -> String {
        let mut response_string =
            format!("{} {} {}\r\n", res.version, res.status_code, res.status_text);
        
        // Handle headers
        if let Some(headers) = &res.headers {
            for (key, value) in headers.iter() {
                response_string.push_str(&format!("{}: {}\r\n", key, value));
            }
        }
        
        // Add Content-Length header if body exists
        if let Some(body) = &res.body {
            response_string.push_str(&format!("Content-Length: {}\r\n", body.len()));
        }

        // End of headers
        response_string.push_str("\r\n");
        
        // Add body if it exists
        if let Some(body) = &res.body {
            response_string.push_str(body);
        }

        response_string
    }
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

    pub fn send(&self, write_stream: &mut impl Write) -> Result<(), std::io::Error> {
        let response_string = String::from(self);
        write!(write_stream, "{}", response_string)?;
        Ok(())
    }

    /// Get the HTTP version
    pub fn version(&self) -> &str {
        self.version
    }
    
    /// Get the status code
    pub fn status_code(&self) -> &str {
        self.status_code
    }
    
    /// Get the status text
    pub fn status_text(&self) -> &str {
        self.status_text
    }
    
    /// Get headers as a formatted string
    pub fn headers(&self) -> String {
        if let Some(headers) = &self.headers {
            let mut headers_string = String::new();
            for (k, v) in headers.iter() {
                headers_string.push_str(&format!("{}: {}\r\n", k, v));
            }
            headers_string
        } else {
            String::new()
        }
    }
    
    /// Get a reference to the headers HashMap
    pub fn headers_map(&self) -> Option<&HashMap<&'a str, &'a str>> {
        self.headers.as_ref()
    }
    
    /// Get the body content
    pub fn body(&self) -> Option<&String> {
        self.body.as_ref()
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

    #[test]
    fn test_response_send() {
        let response = HttpResponse::default();
        let mut buffer = Vec::new();
        let _ = response.send(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        let expected = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_response_with_body() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "text/plain");
        
        let body = Some("Hello, World!".to_string());
        let response = HttpResponse::new("200", Some(headers), body);
        
        let mut buffer = Vec::new();
        let _ = response.send(&mut buffer).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        
        let expected = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 13\r\n\r\nHello, World!";
        assert_eq!(output, expected);
    }

    #[test]
    fn test_from_string_conversion() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type", "application/json");
        headers.insert("Server", "RustHTTP/1.0");
        
        let body = Some("{\"status\":\"success\"}".to_string());
        let response = HttpResponse::new("200", Some(headers), body);
        
        let response_string = String::from(&response);
        
        let expected = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nServer: RustHTTP/1.0\r\nContent-Length: 19\r\n\r\n{\"status\":\"success\"}";
        assert_eq!(response_string, expected);
    }

    #[test]
    fn test_response_404() {
        let response = HttpResponse::new("404", None, Some("Page not found".to_string()));
        let response_string = String::from(&response);
        
        assert!(response_string.contains("HTTP/1.1 404 Not Found"));
        assert!(response_string.contains("Content-Length: 14"));
        assert!(response_string.contains("Page not found"));
    }

    #[test]
    fn test_response_no_body() {
        let mut headers = HashMap::new();
        headers.insert("Location", "/redirect");
        
        let response = HttpResponse::new("302", Some(headers), None);
        let response_string = String::from(&response);
        
        assert!(response_string.contains("HTTP/1.1 302 Unknown"));
        assert!(response_string.contains("Location: /redirect"));
        assert!(!response_string.contains("Content-Length"));
    }
}