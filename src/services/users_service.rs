use anyhow::{anyhow, Context, Result};
use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::result::{Error as DieselError, DatabaseErrorKind};
use uuid::Uuid;
use chrono::{Local,Duration};
use crate::models::{ActivationJson, LoginJson, TokenMail, TokenMailDTO, TokenMailWithUser, User, UserDTO, UserLogin, UserLoginResponse};
use crate::DbPool;
use crate::utils::{AppError, Validation};

pub async fn create_user(pool: &DbPool, new_user: UserDTO) -> Result<TokenMailWithUser, AppError> {
    
    let mut conn = pool.get().map_err(|_| AppError::ConnectionError)?;

    // Validate email
    let email_validation = Validation {
        field: new_user.email.clone()
    };

    if !email_validation.is_valid_email() {
        return Err(AppError::EmailNotValid);
    }

    println!("chegou aqui");
    //Valid Complexity Password
    let password_validation = Validation {
        field: new_user.password.clone()
    };
    if !password_validation.is_valid_password() {
        return Err(AppError::PasswordNotValid);
    }

    // Password hashing
    let password_decrypt = new_user.password.clone();
    let password_hashed = hash(password_decrypt, DEFAULT_COST)
        .map_err(|_| AppError::HashPasswordError)?;
    

    // User recently created has is_actived = false, and then the user must verify the email
    let is_actived = false;
 
    let new_user = UserDTO {
        name: new_user.name,
        email: new_user.email,
        password: password_hashed,
        is_actived: Some(is_actived),
    };

    match User::insert(&mut conn, new_user) {
        Ok(user) => {
            let user_id = user.id.unwrap();
            let other_token = self::send_token_email_verification(&pool, user_id).await?;
            Ok(other_token)
        },
        Err(e) => {
            match e {
                DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    Err(AppError::EmailAlreadyExists)
                },
                _ => {
                    log::info!("Unexpected error: {}", e);
                    Err(AppError::UnexpectedError)
                }
            }
        },
    }

}
pub async fn send_token_email_verification(pool: &DbPool, user_id: i32) -> Result<TokenMailWithUser, AppError> {

    let mut conn = pool.get().map_err(|_| AppError::ConnectionError)?;
    
    // Generate Token with UUID
    let token_mail = Uuid::new_v4().to_string();

    // Token Expires in 1 hour
    let expires_at = Local::now().naive_local() + Duration::hours(1);

    let new_token = TokenMailDTO {
        user_id,
        token: token_mail,
        expires_at,
    };

    match TokenMail::insert(&mut conn, new_token) {
        Ok(token_mail) => {
            log::info!("Token created successfully");
            Ok(token_mail)
        },
        Err(e) => {
            log::info!("Error to insert token: {}", e);
            Err(AppError::UnexpectedError)
        }
    }   

}


pub async fn check_token(pool: &DbPool, activation_info: ActivationJson) -> Result<String, AppError> {
    
    let mut conn = pool.get().map_err(|_| AppError::ConnectionError)?;

    let other_token = activation_info.token;
    let other_email = activation_info.email;


    // Check if token is valid (Email and Token)
    match TokenMail::get_token_by_user_id(&mut conn, other_token, other_email){
        Ok(token_mail) =>{

            let user_id = token_mail.user.id.unwrap();
            let token_mail = token_mail.token.token;

             // If token is valid, check if token is expired
                    // Check datetime


             // If token is not expired, activate the user (updating the user is_actived to true)
                match User::activate_user(&mut conn, user_id){
                    Ok(user) => {
                            if user == 1 {
                                log::info!("User activated successfully");      
                                // Change column used in token for true
                                match TokenMail::update_token_used(&mut conn, token_mail){
                                    Ok(token_mail) => {
                                        if token_mail == 1 {
                                            log::info!("Token updated successfully");
                                            Ok("User activated".to_string())
                                        }else{
                                            log::error!("Error to update token");
                                            return Err(AppError::UnexpectedError)
                                        }
                                    },
                                    Err(e) => {
                                        match e {
                                            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                                                log::error!("Email already exists: {}", e);
                                                return Err(AppError::EmailAlreadyExists)
                                            },  
                                            _ => {
                                                log::error!("Unexpected error: {}", e);
                                                return Err(AppError::UnexpectedError)
                                            }
                                        }
                                    }
                                }
                            }else {
                                log::error!("Error to activate user");
                                return Err(AppError::UnexpectedError)
                            }
                    },
                    Err(e) => {
                        match e {
                            DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                                log::error!("Email already exists: {}", e);
                                return Err(AppError::EmailAlreadyExists)
                            },  
                            _ => {
                                log::error!("Unexpected error: {}", e);
                                return Err(AppError::UnexpectedError)
                            }
                        }
                    }
                }
            
            },
        Err(e) => {
            match e {
                DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    Err(AppError::EmailAlreadyExists)
                },
                _ => {
                    log::error!("Unexpected error: {}", e);
                    Err(AppError::UnexpectedError)
                }
            }
        }
   }

}


pub async fn login_user(pool: &DbPool, login_json: LoginJson) ->  Result<UserLoginResponse, AppError> {
    let mut conn = pool.get().map_err(|_| AppError::ConnectionError)?;


    match User::find_user(&mut conn, login_json.email) {
        Ok(user) => {
            let password_user = &user.password;
            let hash_compare = verify(login_json.password, &password_user).map_err(|_| AppError::HashPasswordError)?;

            if hash_compare {

                Ok(UserLoginResponse {
                    name: user.name,
                    email: user.email,
                 })

            }
            else {
                log::error!("Invalid credentials");
                Err(AppError::InvalidCredentials)
            }

        },
        Err(e) => {
            match e {
                DieselError::NotFound => {
                    log::error!("Invalid credentials: {}", e);
                    Err(AppError::InvalidCredentials)
                },
                _ => {
                    log::error!("Unexpected error: {}", e);
                    Err(AppError::UnexpectedError)
                }
            }
        }   
    }
}
