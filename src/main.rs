#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate diesel;
extern crate dotenv;

#[get("/")]
fn index() -> &'static str {
	"Hello world"
}

fn main() {
	rocket::ignite().mount("/", routes![index]).launch();
}
