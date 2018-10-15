use rocket_contrib::Template;
use std::collections::HashMap;
use database::{connect, get_user};

pub fn launch() {
    rocket::ignite()
        .mount("/", routes![index, user])
        .attach(Template::fairing())
        .catch(catchers![not_found])
        .launch();
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("index", context)
}

#[get("/user/email/<value>")]
fn user(value: String) -> Template {
    match get_user(&connect(), &value) {
        Ok(user) => {
            Template::render("user", user)
        }
        Err(err) => {
            not_found()
        }
    }

}

#[catch(404)]
fn not_found() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("404", context)
}
