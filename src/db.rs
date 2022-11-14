use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{OutCome, Request, State};
use std::ops::Deref;

pub type Pool = r2d2::Poll<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url: String) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db_url")
}