use crate::auth_context::AuthContext;
use rocket::{async_trait, get};
use rocket_dyn_templates::Template;

#[async_trait]
trait IndexTemplate {
    async fn render(&self) -> Template;
}

struct IndexPage;
#[async_trait]
impl IndexTemplate for IndexPage {
    async fn render(&self) -> Template {
        AuthContext::new().render_template("index")
    }
}

#[get("/")]
pub async fn index() -> Template {
    IndexPage.render().await
}
