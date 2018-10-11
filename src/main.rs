#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;

mod database;
mod models;
mod schema;

use diesel::prelude::*;
use database::{connect, create_user, show_users};
use models::NewUser;

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

fn main() {
    let connection = connect();

    let user1 = NewUser {
        email: "blah@fake.com",
        tasks: vec!["bugfix1","newfeature1","improvement1","improvement2"],
        skills: vec!["software development"]
    };

    let user2 = NewUser {
        email: "yah@fake.com",
        tasks: vec!["bugfix2", "bugfix3"],
        skills: vec!["bug fixing"],
    };

    create_user(&connection, &user1);
    create_user(&connection, &user2);

    show_users(&connection);

    rocket::ignite().mount("/", routes![index]).launch();
}
