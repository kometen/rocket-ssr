use rocket::{async_trait, get};
use rocket_dyn_templates::{context, Template};

#[async_trait]
trait AboutTemplate {
    async fn render(&self) -> Template;
}

struct AboutPage;
#[async_trait]
impl AboutTemplate for AboutPage {
    async fn render(&self) -> Template {
        Template::render("about", context! {})
    }
}

#[get("/about")]
pub async fn about() -> Template {
    AboutPage.render().await
}
