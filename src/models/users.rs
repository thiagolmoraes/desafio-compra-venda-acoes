use diesel::{prelude::*, SqliteConnection, Queryable, Insertable, Selectable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::users::{self, dsl::*};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginJson {
    pub email: String,
    pub password: String,
}


#[derive(Queryable, Selectable, Deserialize, Debug, Serialize)]
#[diesel(table_name = users)]
pub struct UserLogin {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserLoginResponse {
    pub email: String,
    pub name: String,
}

#[derive(Queryable, Selectable, Deserialize, Debug, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    // pub password: String,
    // pub is_actived: Option<bool>,
    // pub created_at: Option<NaiveDateTime>,
    // pub updated_at: Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_actived: Option<bool>,
}

impl User {
    pub fn insert(conn: &mut SqliteConnection, new_user: UserDTO) -> QueryResult<Self> {
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;
           
        users::table
            .order(users::id.desc())
            .select(User::as_select())
            .first(conn)
    }

    pub fn activate_user(conn: &mut SqliteConnection, other_user_id: i32) -> QueryResult<usize> {
        diesel::update(users::table)
           .filter(users::id.eq(other_user_id))
           .set(users::is_actived.eq(true))
           .execute(conn)
    } 

    pub fn find_user(conn: &mut SqliteConnection, other_email: String) -> QueryResult<UserLogin> {
        users::table 
            .filter(users::email.eq(other_email))
            .filter(users::is_actived.eq(true))
            .select(UserLogin::as_select())
            .first(conn)
    }



}
