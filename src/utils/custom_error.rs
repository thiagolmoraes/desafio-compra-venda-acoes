use thiserror::Error;
use diesel::result::Error as DieselError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Error to connect to the database")]
    ConnectionError(#[from] DieselError),

    #[error("Error to hash password")]
    HashPasswordError,

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Erro inesperado: {0}")]
    UnexpectedError(#[from] anyhow::Error),

    #[error("Email is not valid")]
    EmailNotValid,

    #[error("Your password is too weak. It should be at least 12 characters long and include a mix of uppercase and lowercase letters, numbers, and special characters to ensure better security.")]
    PasswordNotValid
}
