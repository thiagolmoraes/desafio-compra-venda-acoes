use diesel::{prelude::*, SqliteConnection, Queryable, Insertable, Selectable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::users::{self, dsl::*};



#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    // pub id: Option<i32>,
    pub name: String,
    pub email: String,
    // pub password: String,
    // pub is_actived: Option<bool>,
    // pub created_at: Option<NaiveDateTime>,
    // pub updated_at: Option<NaiveDateTime>
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub name: String,
    pub email: String,
    pub password: String,
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
}
