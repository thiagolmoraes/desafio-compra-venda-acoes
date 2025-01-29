use thiserror::Error;
use serde_json::Error as SerdeError;
use diesel::result::Error as DieselError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error to connect to the database")]
    ConnectionError,

    #[error("Error to hash password")]
    HashPasswordError,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Unexpected Error")]
    UnexpectedError,

    #[error("Email is not valid")]
    EmailNotValid,

    #[error("Your password is too weak. It should be at least 12 characters long and include a mix of uppercase and lowercase letters, numbers, and special characters to ensure better security.")]
    PasswordNotValid,

    #[error("Time Token was expired")]
    ExpiredToken,

    #[error("Token is not valid")]
    InvalidToken,

    #[error("Email or Password is not valid")]
    InvalidCredentials,

}
