use rocket::http::Status;
use rocket::local::blocking::Client;

use super::rocket;

#[test]
fn hello_test() {
    let client = Client::tracked(rocket()).expect("valid rocket");
    let response = client.get("/test").dispatch();

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.headers().get_one("Content-Type"),
        Some("text/html; charset=utf-8")
    );
    assert_eq!(
        response.into_string().map(|s| s.trim_end().to_string()),
        Some("<b>Hello, test!</b>".into())
    );
}
