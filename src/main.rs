#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern  crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate roxket;
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

