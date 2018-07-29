#![feature(plugin)]
#![plugin(rocket_codegen)]
#[cfg(test)] mod tests;

extern crate rocket;
use rocket::Rocket;

#[get("/")]
fn index() -> &'static str {
    "The Anti-Social Network"
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![index])
}

fn main() {
    rocket().launch();
}
