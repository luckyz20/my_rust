//From Server To handle
use super::handler::*;
use http::{
    httprequest::{HttpRequest, Method, Resource},
    httpresponse::HttpResponse,
};
use std::io::Write;
pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            Method::Get => match &req.resource {
                Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
                _ => {
                    //other path
                    let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                    let _ = resp.send_response(stream);
                }
            },
            Method::Post => {}
            _ => {}
        }
    }
}
