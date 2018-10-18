use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::{User, NewUser};
use schema::users;

pub fn connect() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Cannot find DATABASE_URL in .env file");
    PgConnection::establish(&database_url).expect("Error connecting to database")
}

pub fn show_users(connection: &PgConnection) {
    use schema::users::dsl::*;
    let users_data = users.load::<User>(connection).expect("blah");

    for user in users_data {
        println!("{:?}\n", user);
    }
}

pub fn get_user(connection: &PgConnection, email: &str) -> Result<User, diesel::result::Error> {
    users::table.filter(users::email.eq(email))
                .first::<User>(connection)
}

pub fn create_user(connection: &PgConnection, user: &NewUser) -> Result<User, diesel::result::Error> {
    use schema::users::dsl::*;

    diesel::insert_into(users)
        .values(user)
        .get_result::<User>(connection)
}
