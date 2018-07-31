#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[cfg(test)] mod tests;

use rocket::response::NamedFile;
use rocket::response::Redirect;
use rocket::request::{Form, FromFormValue};
use rocket::http::RawStr;
use std::io;

#[derive(Debug)]
struct StrongPassword<'r>(&'r str);

#[derive(FromForm)]
struct UserLogin<'r> {
    username: &'r RawStr,
    password: Result<StrongPassword<'r>, &'static str>,
}

impl<'v> FromFormValue<'v> for StrongPassword<'v> {
    type Error = &'static str;

    fn from_form_value(v: &'v RawStr) -> Result<Self, Self::Error> {
        if v.len() < 10 {
            Err("Password must be at least 10 characters.")
        } else {
            Ok(StrongPassword(v.as_str()))
        }
    }
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/login")]
fn login_page() -> io::Result<NamedFile> {
    NamedFile::open("static/login.html")
}

#[post("/login", data = "<user_form>")]
fn login<'a>(user_form: Form<'a, UserLogin<'a>>) -> Result<Redirect, String> {
    let user = user_form.get();

    if let Err(e) = user.password {
        return Err("Incorrect Username or Password".to_string());
    }

    // TODO: use real database for this later.
    if user.username == "Luke" {
        if let Ok(StrongPassword("password01")) = user.password {
            Ok(Redirect::to("/user/Luke"))
        } else {
            Err("Incorrect Username or Password".to_string())
        }
    } else {
        Err("Incorrect Username or Password".to_string())
    }
}

#[get("/user/<username>")]
fn user_page(username: &RawStr) -> String {
    format!("This is {}'s page.", username)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, user_page, login_page, login])
}

fn main() {
    rocket().launch();
}
