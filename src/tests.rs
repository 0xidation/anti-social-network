use super::rocket;
use rocket::local::Client;
use rocket::http::{ContentType, Status};

#[test]
fn test_index() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response.body_string().is_some());
}

fn test_login<T>(user: &str, pass: &str, status: Status, body: T)
    where T: Into<Option<&'static str>>
{
    let client = Client::new(rocket()).unwrap();
    let query = format!("username={}&password={}", user, pass);
    let mut response = client.post("/login")
        .header(ContentType::Form)
        .body(&query)
        .dispatch();

    assert_eq!(response.status(), status);
    if let Some(expected_str) = body.into() {
        let body_str = response.body_string();
        assert!(body_str.map_or(false, |s| s.contains(expected_str)));
    }
}

#[test]
fn test_good_login() {
    test_login("Luke", "password10", Status::Ok, None);
}

#[test]
fn test_invalid_user() {
    test_login("-1", "password10", Status::Ok, "Incorrect Username or Password");
    test_login("Mike", "password10", Status::Ok, "Incorrect Username or Password");
}

#[test]
fn test_invalid_password() {
    test_login("Luke", "password101", Status::Ok, "Incorrect Username or Password");
    test_login("Luke", "ok", Status::Ok, "Incorrect Username or Password");
}

fn check_bad_form(form_str: &str, status: Status) {
    let client = Client::new(rocket()).unwrap();
    let response = client.post("/login")
        .header(ContentType::Form)
        .body(form_str)
        .dispatch();

    assert_eq!(response.status(), status);
}

#[test]
fn test_bad_form_abnromal_inputs() {
    check_bad_form("&&&===&", Status::BadRequest);
    check_bad_form("&&&=hi==&", Status::BadRequest);
}

#[test]
fn test_bad_form_missing_fields() {
    let bad_inputs: [&str; 2] = [
        "&",
        "=",
    ];

    for bad_input in bad_inputs.into_iter() {
        check_bad_form(bad_input, Status::BadRequest);
    }
}

#[test]
fn test_bad_form_additional_fields() {
    check_bad_form("username=Luke&password=pass&addition=1",
                   Status::UnprocessableEntity);
}
