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

pub fn update_user(connection: &PgConnection, user: &User) -> Result<User, diesel::result::Error> {
    diesel::update(
            users::table.filter(
                users::id.eq(&user.id)))
        .set(
            (users::email.eq(&user.email),
             users::skills.eq(&user.skills),
             users::tasks.eq(&user.tasks)))
        .get_result::<User>(connection)
}


mod tests {
    #[test]
    fn create_user_test() {
        use database::connect;
        use diesel::prelude::*;
        use models::{User,NewUser};
        use database::create_user;

        let connection = connect();

        let test_user = NewUser {
            email: "iuhewdiunasidai@fake.com",
            tasks: vec!["bugfix1","improvement1"],
            skills: vec!["software development"],
        };

        let expected_test_user = User {
            id: 1,
            email: "iuhewdiunasidai@fake.com".to_string(),
            tasks: Some(vec!["bugfix1".to_string(),"improvement1".to_string()]),
            skills: Some(vec!["software development".to_string()]),
        };

        connection.test_transaction::<_, diesel::result::Error, _>(|| {
            let created_test_user = create_user(&connection, &test_user).expect("Test user creation failed");

            if expected_test_user.email != created_test_user.email &&
               expected_test_user.tasks != created_test_user.tasks &&
               expected_test_user.skills != created_test_user.skills {
                Err(diesel::result::Error::__Nonexhaustive)
            } else {
                Ok(())
            }
        })
    }

    #[test]
    fn repeat_create_test() {
        use database::connect;
        use diesel::prelude::*;
        use models::NewUser;
        use database::create_user;

        let connection = connect();

        let test_user = NewUser {
            email: "iojdoiajsoidjascod@fake.com",
            tasks: vec!["bugfix1","improvement1"],
            skills: vec!["software development"],
        };

        connection.test_transaction::<_, diesel::result::Error, _>(|| {
            let first_create_test = create_user(&connection, &test_user);
            let second_create_test = create_user(&connection, &test_user);

            if first_create_test.is_err() && second_create_test.is_err() {
                Err(diesel::result::Error::__Nonexhaustive)
            } else {
                Ok(())
            }
        })
    }

    // #[test]
    // fn get_user_test() {
    // }

    // #[test]
    // fn update_user_test() {
    // }
}
