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
impl User {

    pub fn get_all_users(conn: &PgConnection) -> Vec<User> {
        all_users
            .order(users::id.desc())
            .load::<User>(conn)
            .expect("error!")
    }

    pub fn insert_user(user: NewUser, conn: &PgConnection) -> bool {
        diesel::insert_into(users::table)
            .values(&user)
            .execute(conn)
            .is_ok()
    }

    pub fn get_user_by_username(user: UserData, conn: & PgConnection) -> Vec<User>{
        all_users
            .filter(users::username.eq(user.username))
            .load::<User>(conn)
            .expect("error!")
    }
}