use crate::auth_context::AuthContext;
use rocket::{async_trait, get};
use rocket_dyn_templates::Template;

#[async_trait]
trait AboutTemplate {
    async fn render(&self) -> Template;
}

struct AboutPage;
#[async_trait]
impl AboutTemplate for AboutPage {
    async fn render(&self) -> Template {
        AuthContext::new().render_template("about")
    }
}

#[get("/about")]
pub async fn about() -> Template {
    AboutPage.render().await
}
