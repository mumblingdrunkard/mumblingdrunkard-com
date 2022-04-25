use actix_web::{get, web, App, HttpServer, Responder};
use serde_derive::Serialize;

#[derive(Serialize)]
struct MyResponse {
    status: &'static str,
}

#[get("/")]
async fn api() -> impl Responder {
    web::Json(MyResponse { status: "success" })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api").service(api)))
        .bind(("0.0.0.0", 8082))?
        .run()
        .await
}
