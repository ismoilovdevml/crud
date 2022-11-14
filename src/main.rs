#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern  crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate serde_json;

use dotenv::dotenv;
use std::env;
use routes::*;
use std::process::Command;

mod db;
mod models;
mod routes;
mod schema;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    


    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1",
            routes![get_all, new_user, find_user],
        )
}

fn main() {
    let _output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cd ui && npm start"])
            .spawn()
            .expect("UI ilovasini ishga tushirib bo'lmadi")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("cd ui && npm start")
            .spawn()
            .expect("UI ilovasini ishga tushirib bo'lmadi")
    };
    rocket().launch();
}

