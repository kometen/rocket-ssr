use rocket::{async_trait, get};
use rocket_dyn_templates::{context, Template};

#[async_trait]
trait TestTemplate {
    async fn render(&self) -> Template;
}

struct TestPage;
#[async_trait]
impl TestTemplate for TestPage {
    async fn render(&self) -> Template {
        Template::render("test", context! {})
    }
}

#[get("/test")]
pub async fn test() -> Template {
    TestPage.render().await
}

#[cfg(test)]
mod tests {

    use rocket::{http::Status, local::blocking::Client, routes};

    #[test]
    fn hello_test() {
        // Create a test context instead of using super::rocket()
        let rocket_instance = rocket::build().mount("/", routes![super::test]);
        let client = Client::tracked(rocket_instance).expect("valid rocket");
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
}
