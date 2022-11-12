use diesel;
use serde_derive::Serialize;
use serde_derive::Deserialize;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use super::schema::users;
use super::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub usernmae: String,
    pub password: String,
    pub frist_name: String,
}

#[derive(Deserialize)]
pub struct UserData{
    pub username: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub usernmae: String,
    pub password: String,
    pub frist_name: String
}