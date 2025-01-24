use actix_web::{get, HttpResponse };

#[get("/hello")]
pub async fn handler_hello() -> HttpResponse  {
    HttpResponse::Ok().body("Hello, World!")
}