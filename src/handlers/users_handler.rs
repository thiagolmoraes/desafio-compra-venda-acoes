use actix_web::{HttpResponse, routes, web};
use serde_json::json;
use crate::{DbPool, models::UserDTO, services::create_user};
use crate::utils::AppError;

#[routes]
#[post("/users")]
pub async fn create_user_handler(
    pool: web::Data<DbPool>,
    new_user: web::Json<UserDTO>
) -> HttpResponse {
    match create_user(&pool, new_user.into_inner()).await {
        Ok(_) => HttpResponse::Created().json(json!({"message": "User created successfully, you will receive a confirmation email"})),
        Err(e) => {
            let response = json!({ "message": e.to_string() });
            match e {
                AppError::EmailAlreadyExists => HttpResponse::Conflict().json(response),
                AppError::UnexpectedError(_) => HttpResponse::InternalServerError().json(response),
                AppError::HashPasswordError => HttpResponse::InternalServerError().json(response),
                AppError::EmailNotValid => HttpResponse::BadRequest().json(response),
                AppError::PasswordNotValid => HttpResponse::BadRequest().json(response),
                _ => HttpResponse::InternalServerError().json(response),
            }
        }
    }
}


#[routes]
#[get("/users")]
pub async fn test_user() -> HttpResponse {
    HttpResponse::Ok().body("Teste")
}