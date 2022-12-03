use actix_web::{HttpServer, web, HttpResponse};

mod executor {
    pub mod validate_password;
}