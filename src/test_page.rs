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
