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
    r2d2::Pool::new(manager).expect("db pool failure")
}
pub struct Conn(pub r2d2::PoolConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Conn{
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::OutCome<Conn, ()>{
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => OutCome::Success(Conn(conn)),
            Err(_) => OutCome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = PgConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}