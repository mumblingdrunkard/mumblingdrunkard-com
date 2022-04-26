use actix_web::{get, web, App, HttpServer, Responder};
use serde_derive::Serialize;
use std::fs;

#[derive(Serialize)]
struct MyResponse {
    status: &'static str,
    content: String,
}

#[get("/test")]
async fn api() -> impl Responder {
    web::Json(MyResponse {
        status: "success",
        content: fs::read_to_string("test.html").unwrap(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api").service(api)))
        .bind(("0.0.0.0", 8082))?
        .run()
        .await
}
