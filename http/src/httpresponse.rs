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
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
    // add code here
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
}
