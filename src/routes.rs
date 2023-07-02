```rust
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::controllers::{get_ascii_duck, send_message_to_model};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/ascii-duck")]
async fn ascii_duck() -> impl Responder {
    let duck = get_ascii_duck().await;
    HttpResponse::Ok().body(duck)
}

#[post("/send-message")]
async fn send_message(message: web::Json<String>) -> impl Responder {
    let response = send_message_to_model(message.into_inner()).await;
    HttpResponse::Ok().json(response)
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.service(ascii_duck);
    cfg.service(send_message);
}
```