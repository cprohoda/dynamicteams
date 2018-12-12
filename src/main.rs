#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;

mod database;
mod models;
mod schema;
mod web;

use diesel::prelude::*;
use database::{connect, create_user, show_users, get_user, update_user};
use models::NewUser;
use models::User;

fn main() {
    web::launch();
}
