use std::io;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

//Route cfg
pub fn general_route(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

//Handler cfg
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web Service is running")
}

//instance HTTP Server and run
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // build app, route cfg
    let app = move || App::new().configure(general_route);

    //run HTTP Server
    HttpServer::new(app).bind("127.0.0.1:8088")?.run().await
}
