use super::rocket;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn index_test() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("The Anti-Social Network".into()));
}
