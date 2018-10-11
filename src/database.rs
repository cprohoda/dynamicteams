use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::{User, NewUser};

pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Cannot find DATABASE_URL in .env file");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

pub fn show_users(connection: &PgConnection) {
    use schema::users::dsl::*;
    let users_data = users.load::<User>(connection).expect("blah");

    for user in users_data {
        println!("{:?}", user.id);
        println!("{:?}", user.email);
        println!("{:?}", user.skills);
        println!("{:?}", user.tasks);
        println!("\n");
    }
}

pub fn create_user(connection: &PgConnection, user: &NewUser) {
    use schema::users::dsl::*;

    diesel::insert_into(users)
        .values(user)
        .get_result::<User>(connection)
        .expect("Error saving new users");
}
