use actix_web::{post, web, HttpResponse, Responder};
use log::info;
use pam::Authenticator;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Request {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct Response {
    result: bool,
}

#[post("validate_password")]
pub async fn validate_password(request: web::Json<Request>) -> impl Responder {
    info!("{} uchun parol tasdiqlanmoqda", request.username);

    if authenticate(&request.username, &request.password) {
        HttpResponse::Ok().json(Response { result: true })
    } else {
        HttpResponse::Ok().json(Response { result: false })
    }
}

pub fn authenticate(username: &str, password: &str) -> bool {
    let mut authenticator = 
        Authenticator::with_password("login").expect("Client ishga tushmadi");

    authenticator
        .get_handler()
        .set_credentails(username, password);
    authenticator.authenticate().is_ok()
}