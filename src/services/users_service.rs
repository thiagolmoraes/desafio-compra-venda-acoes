use anyhow::{anyhow, Context, Result};
use bcrypt::{DEFAULT_COST, hash, verify};
use diesel::result::{Error as DieselError, DatabaseErrorKind};

use crate::models::{UserDTO, User};
use crate::DbPool;
use crate::utils::{AppError, Validation};

pub async fn create_user(pool: &DbPool, new_user: UserDTO) -> Result<User, AppError> {
    
    let mut conn = pool.get().context("Error to connect to the database")?;

    // Validate email
    let email_validation = Validation {
        field: new_user.email.clone()
    };

    if !email_validation.is_valid_email() {
        return Err(AppError::EmailNotValid);
    }


    //Valid Complexity Password
    let password_validation = Validation {
        field: new_user.password.clone()
    };
    if !password_validation.is_valid_password() {
        return Err(AppError::PasswordNotValid);
    }

    // Password hashing
    let password_decrypt = new_user.password.clone();
    let password_hashed = hash( password_decrypt, DEFAULT_COST)
        .map_err(|_| AppError::HashPasswordError)?;
    

 
    let new_user = UserDTO {
        name: new_user.name,
        email: new_user.email,
        password: password_hashed,
    };

    match User::insert(&mut conn, new_user) {
        Ok(user) => Ok(user),
        Err(e) => {
            match e {
                DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    Err(AppError::EmailAlreadyExists)
                },
                _ => {
                    Err(AppError::UnexpectedError(e.into()))
                }
            }
        },
    }

}

//pub async verify_user_code() {} 



