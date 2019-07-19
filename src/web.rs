use rocket_contrib::templates::Template;
use std::collections::HashMap;
use database::{connect, get_user, get_org};

pub fn launch() {
    rocket::ignite()
        .mount("/", routes![index, user, org])
        .attach(Template::fairing())
        .register(catchers![not_found])
        .launch();
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("index", context)
}

// #[post("/login")]
// fn ()

#[get("/user/email/<value>")]
fn user(value: String) -> Template {
    match get_user(&connect(), &value) {
        Ok(user) => {
            Template::render("user", user)
        }
        Err(_err) => {
            not_found()
        }
    }
}

#[get("/org/<value>")]
fn org(value: String) -> Template {
    match get_org(&connect(), &value) {
        Ok(org) => {
            Template::render("org", org)
        }
        Err(_err) => {
            not_found()
        }
    }
}

#[catch(404)]
fn not_found() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("404", context)
}
