use actix_web::{HttpResponse, routes, web};
use serde_json::json;
use crate::{DbPool, models::{UserDTO, ActivationJson, LoginJson}, services::{create_user, check_token, login_user}};
use crate::utils::AppError;

#[routes]
#[post("/users")]
pub async fn create_user_handler(
    pool: web::Data<DbPool>,
    new_user: web::Json<UserDTO>
) -> HttpResponse {
    match create_user(&pool, new_user.into_inner()).await {
        Ok(token_user) => {
        HttpResponse::Created().json(json!({
                "message": "User created successfully, you will receive a confirmation email",
                "token": token_user.token.token,
                "expires_at": token_user.token.expires_at
            }))
        },
        Err(e) => {
            let response = json!({ "message": e.to_string() });
            match e {
                AppError::EmailAlreadyExists => HttpResponse::Conflict().json(response),
                AppError::UnexpectedError => HttpResponse::InternalServerError().json(response),
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

#[routes]
#[post("/user/activate")]
pub async fn activate_user(
     pool: web::Data<DbPool>,
     activation_info: web::Json<ActivationJson>,
    ) -> HttpResponse {

    let activation_info = activation_info.into_inner();

    match check_token(&pool, activation_info).await {
        Ok(info)=> {
            // HttpResponse::Ok().body("User activated successfully")            
            HttpResponse::Ok().json(info)            
        },
        Err(e) => {
            let response = json!({ "message": e.to_string() });
            match e {
                AppError::ExpiredToken => HttpResponse::Unauthorized().json(response),
                AppError::UnexpectedError => HttpResponse::InternalServerError().json(response),
                AppError::InvalidToken => HttpResponse::NotFound().json(response),
                _ => HttpResponse::InternalServerError().json(response),
            }
        }
    }    

}


#[routes]
#[post("/login")]
pub async fn login_user_handler(
    pool: web::Data<DbPool>,
    login_json: web::Json<LoginJson>
) -> HttpResponse {
    
    match login_user(&pool, login_json.into_inner()).await {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        },
        Err(e) => {
            let response = json!({ "message": e.to_string() });
            match e {
                AppError::InvalidCredentials => HttpResponse::Unauthorized().json(response),
                AppError::UnexpectedError => HttpResponse::InternalServerError().json(response),
                _ => HttpResponse::InternalServerError().json(response),
            }
        }
}

}