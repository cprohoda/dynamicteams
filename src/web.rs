#![feature(plugin)]
#![plugin(rocket_codegen)]

fn launch() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}
