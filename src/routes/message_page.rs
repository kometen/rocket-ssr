use crate::auth::{
    auth_context::AuthContext,
    request_guard::{LimitedId, User},
};
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

#[get("/message/<id>")]
pub async fn view_message(id: LimitedId, user: User) -> Template {
    let mut context = AuthContext::new(user.0);
    context.insert("id", &id.0);
    context.render_template("view_message")
}
