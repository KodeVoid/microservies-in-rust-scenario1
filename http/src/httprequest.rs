use std::collections::HashMap;
// Method enum for method type of httprequest
#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Unintialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Unintialized,
        }
    }
}

//Version enum for http version
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Unintialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Unintialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}
impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method: Method = "".into();
        let mut parsed_version = Version::V1_1;
        let mut parsed_resources = Resource::Path("".into());
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = "";

        // read each line in the incomming request
        for line in req.lines() {
            // if line is request line call fn process_req_line
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resources = resource;
                parsed_version = version
            }
            // if the line is header call process header
            else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            }
            // else if blank line do nothing
            else if line.len() == 0 {
            } else {
                parsed_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            resource: parsed_resources,
            version: parsed_version,
            headers: parsed_headers,
            body: parsed_body.into(),
        }
    }
}
fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(resource.into()),
        version.into(),
    )
}
fn process_header_line(s: &str) -> (String, String) {
    // parse the header line into words split by seperator ":"
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header_items.next() {
        key = k.trim().to_string()
    }
    if let Some(v) = header_items.next() {
        value = v.trim().to_string()
    }
    (key, value)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        let n: Method = "POST".into();
        let u: Method = "Put".into();
        assert_eq!(m, Method::Get);
        assert_eq!(n, Method::Post);
        assert_eq!(u, Method::Unintialized);
    }

    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }
    #[test]
    fn test_http_request_from() {
        let raw = "GET /index.html HTTP/1.1\r\nHost: example.com\r\n\r\nBody content";
        let req: HttpRequest = raw.to_string().into();

        assert_eq!(req.method, Method::Get);
        assert_eq!(req.version, Version::V1_1);
        assert_eq!(req.resource, Resource::Path("/index.html".to_string()));
        assert_eq!(req.headers.get("Host").unwrap(), "example.com");
        assert_eq!(req.body, "Body content");
    }
}
