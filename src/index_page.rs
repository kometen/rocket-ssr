use rocket::{async_trait, get};
use rocket_dyn_templates::{context, Template};

#[async_trait]
trait IndexTemplate {
    async fn render(&self) -> Template;
}

struct IndexPage;
#[async_trait]
impl IndexTemplate for IndexPage {
    async fn render(&self) -> Template {
        Template::render("index", context! {})
    }
}

#[get("/")]
pub async fn index() -> Template {
    IndexPage.render().await
}
