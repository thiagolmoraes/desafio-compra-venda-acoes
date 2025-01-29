use diesel::{prelude::*, SqliteConnection, Queryable, Insertable, Selectable};
use serde::{Deserialize, Serialize};
use crate::schema::{email_verification_tokens::{self, dsl::*}, users::{self, dsl::*}};
use crate::models::User;
use chrono::NaiveDateTime;

// Handler Struct for Activation Email
#[derive(Debug, Deserialize, Serialize)]
pub struct ActivationJson {
    pub token: String,
    pub email: String
}


#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = email_verification_tokens)]
pub struct TokenMail {
    pub token: String,
    pub expires_at: NaiveDateTime,
    // pub user_id: i32
}

#[derive(Queryable, Debug, Serialize)]
pub struct TokenMailWithUser {
    pub token: TokenMail,
    pub user: User
}

#[derive(Debug, Insertable)]
#[diesel(table_name = email_verification_tokens)]
pub struct TokenMailDTO{
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub user_id: i32
}


impl TokenMail {
  pub fn insert(conn: &mut SqliteConnection, new_token: TokenMailDTO) -> QueryResult<TokenMailWithUser> {
      diesel::insert_into(email_verification_tokens::table)
          .values(&new_token)
          .execute(conn)?;

      email_verification_tokens::table
          .inner_join(users::table)
          .order(email_verification_tokens::id.desc())
          .select((TokenMail::as_select(), User::as_select()))
          .first(conn)
  }


  pub fn get_token_by_user_id(conn: &mut SqliteConnection, other_token: String, other_email: String) -> QueryResult<TokenMailWithUser> {
      email_verification_tokens::table
          .inner_join(users::table)
          .filter(users::email.eq(other_email))
          .filter(email_verification_tokens::used.eq(false))
          .filter(email_verification_tokens::token.eq(other_token))
          .select((TokenMail::as_select(), User::as_select()))
          .first(conn)
  }

  pub fn update_token_used(conn: &mut SqliteConnection, other_token: String) -> QueryResult<usize> {
        diesel::update(email_verification_tokens::table)
            .filter(email_verification_tokens::token.eq(other_token))
            .set(email_verification_tokens::used.eq(true))
            .execute(conn)
  }



}