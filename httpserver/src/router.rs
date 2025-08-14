use super::handlers::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use std::io::Write;
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) {
        match req.method {
            // GET request
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();
                    match route.get(1).copied().unwrap_or("") {
                        // if route begins with /api invoke Web service
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send(stream);
                        }
                        // else invoke static page handler
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send(stream);
                        }
                    }
                }
            },
            // Any other method -> 404
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send(stream);
            }
        }
    }
}
