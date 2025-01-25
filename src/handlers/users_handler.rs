use actix_web::{HttpResponse, routes, web};
use crate::{DbPool, models::UserDTO, services::create_user};

#[routes]
#[post("/users")]
pub async fn create_user_handler(
    pool: web::Data<DbPool>,
    new_user: web::Json<UserDTO>
) -> HttpResponse {
    match create_user(&pool, new_user.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::InternalServerError().json(format!("Erro ao criar usuário: {}", e))
    }
}


#[routes]
#[get("/users")]
pub async fn test_user() -> HttpResponse {
    HttpResponse::Ok().body("Teste")
}