use anyhow::{anyhow, Context, Result};
use crate::models::{UserDTO, User};
use crate::DbPool;

pub async fn create_user(pool: &DbPool, new_user: UserDTO) -> Result<User> {
    
    let mut conn = pool.get().context("Error to connect to the database")?;
    
    match User::insert(&mut conn, new_user) {
        Ok(user) => Ok(user),
        Err(e) => Err(anyhow!("Error to Create User: {}", e)),
    }


}




