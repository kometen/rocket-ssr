use crate::{auth_context::AuthContext, request_guard::User};
use rocket::{async_trait, get};
use rocket_dyn_templates::Template;

#[async_trait]
trait IndexTemplate {
    async fn render(&self, user: User) -> Template;
}

struct IndexPage;
#[async_trait]
impl IndexTemplate for IndexPage {
    async fn render(&self, user: User) -> Template {
        AuthContext::new(user.0).render_template("index")
    }
}

#[get("/")]
pub async fn index(user: User) -> Template {
    IndexPage.render(user).await
}
