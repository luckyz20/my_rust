use std::{collections::HashMap, env, fs};

use http::{
    httprequest::{HttpRequest, Resource},
    httpresponse::HttpResponse,
};
use serde::{Deserialize, Serialize};

pub trait Handle {
    fn handle(req: &HttpRequest) -> HttpResponse;

    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);
        contents.ok()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

pub struct WebServiceHandler;
pub struct StaticPageHandler;
pub struct PageNotFoundHandler;

impl WebServiceHandler {
    pub fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "order.json");
        let json_contents = fs::read_to_string(full_path).unwrap();
        let json_data: Vec<OrderStatus> = serde_json::from_str(json_contents.as_str()).unwrap();
        json_data
    }

    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::default()
    }
}
impl Handle for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        //localhost:8088/api/shipping/orders
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = serde_json::to_string(&Self::load_json()).unwrap();
                let mut headers = HashMap::new();
                headers.insert("Content-Type", " application/json");
                HttpResponse::new("200", Some(headers), Some(body))
            }
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        }
    }
}

//
impl Handle for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();

        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),

            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),

            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map = HashMap::new();
                    if contents.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if contents.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        }
    }
}

//
impl Handle for PageNotFoundHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}
