use crate::{auth_context::AuthContext, request_guard::User};
use rocket::{async_trait, get};
use rocket_dyn_templates::Template;

#[async_trait]
trait MessageTemplate {
    async fn render(&self, user: User) -> Template;
}

struct MessagePage;
#[async_trait]
impl MessageTemplate for MessagePage {
    async fn render(&self, user: User) -> Template {
        AuthContext::new(user.0).render_template("message")
    }
}

#[get("/message")]
pub async fn message(user: User) -> Template {
    MessagePage.render(user).await
}
