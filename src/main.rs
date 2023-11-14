pub mod util;

use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use util::time_in_seconds;
use uuid::Uuid;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("API is up and Running!")
}

#[derive(Debug, Serialize)]
struct AuthResponse {
    token: String,
    expire: i64,
    signature: String,
}

#[get("/imagekit")]
async fn imagekit_handler() -> impl Responder {
    let auth_res = AuthResponse {
        token: Uuid::new_v4().to_string(),
        expire: time_in_seconds(),
        signature: "0f9d5a45e97c24fa9200a9d5543c9af1e2c45a54".to_string(),
    };

    HttpResponse::Ok().json(auth_res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().send_wildcard();
        App::new()
            .wrap(cors)
            .service(imagekit_handler)
            .service(health)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
