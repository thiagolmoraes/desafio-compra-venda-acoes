use diesel::{prelude::*, SqliteConnection, Queryable, Insertable, Selectable};
use serde::{Deserialize, Serialize};
use crate::schema::email_verification_tokens::{self, dsl::*};
use chrono::NaiveDateTime;


#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = email_verification_tokens)]
pub struct TokenMail {
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub user_id: i32
}

#[derive(Debug, Insertable)]
#[diesel(table_name = email_verification_tokens)]
pub struct TokenMailDTO{
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub user_id: i32
}

impl TokenMail {
    pub fn insert(conn: &mut SqliteConnection, new_token: TokenMailDTO) -> QueryResult<Self> {
        diesel::insert_into(email_verification_tokens::table)
            .values(&new_token)
            .execute(conn)?;

        email_verification_tokens::table
          .order(email_verification_tokens::id.desc())
          .select(TokenMail::as_select())
          .first(conn)
          

    }
}