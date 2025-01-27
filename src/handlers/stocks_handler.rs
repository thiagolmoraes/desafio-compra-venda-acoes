use actix_web::{HttpResponse, routes};


#[routes]
#[get("/stocks")]
pub async fn get_stocks() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to the Stocks API!")
}