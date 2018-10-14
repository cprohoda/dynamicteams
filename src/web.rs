use rocket_contrib::Template;
use std::collections::HashMap;

pub fn launch() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .catch(catchers![not_found])
        .launch();
}

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("index", context)
}

#[catch(404)]
fn not_found() -> Template {
    let context: HashMap<String, String> = HashMap::new();

    Template::render("404", context)
}
