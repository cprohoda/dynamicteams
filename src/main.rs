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
    let connection = connect();

    let user1 = NewUser {
        email: "blah@fake.com",
        tasks: vec!["bugfix1","newfeature1","improvement1","improvement2"],
        skills: vec!["software development"],
    };

    let user2 = NewUser {
        email: "yah@fake.com",
        tasks: vec!["bugfix2", "bugfix3"],
        skills: vec!["bug fixing"],
    };

    create_user(&connection, &user1);
    create_user(&connection, &user2);

    show_users(&connection);

    let first_user = get_user(&connection, user1.email).expect("");
    println!("First user: {:?}", first_user);

    let mut updated_user1 = User {
        id: first_user.id,
        email: "blah@fake.com".to_string(),
        tasks: Some(vec!["bugfix1".to_string(),"newfeature1".to_string(),"improvement1".to_string(),"improvement2".to_string()]),
        skills: Some(vec!["software development".to_string(),"flight".to_string()]),
    };
    println!("Update: {:?}", update_user(&connection, &updated_user1));

    let updated_first_user = get_user(&connection, updated_user1.email.as_str()).expect("");
    println!("Updated first user: {:?}", updated_first_user);

    web::launch();
}
