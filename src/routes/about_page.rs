use crate::{auth::auth_context::AuthContext, auth::request_guard::User};
use rocket::{async_trait, get};
use rocket_dyn_templates::Template;

#[async_trait]
trait AboutTemplate {
    async fn render(&self, user: User) -> Template;
}

struct AboutPage;
#[async_trait]
impl AboutTemplate for AboutPage {
    async fn render(&self, user: User) -> Template {
        AuthContext::new(user.0).render_template("about")
    }
}

#[get("/about")]
pub async fn about(user: User) -> Template {
    AboutPage.render(user).await
}
