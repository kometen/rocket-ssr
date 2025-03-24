use crate::{auth::auth_context::AuthContext, auth::request_guard::User};
use rocket::{async_trait, get};
use rocket_dyn_templates::Template;

#[async_trait]
trait RegisterAccountTemplate {
    async fn render(&self, user: User) -> Template;
}

struct RegisterAccountPage;
#[async_trait]
impl RegisterAccountTemplate for RegisterAccountPage {
    async fn render(&self, user: User) -> Template {
        AuthContext::new(user.0).render_template("register_account")
    }
}

#[get("/register")]
pub async fn register_account(user: User) -> Template {
    RegisterAccountPage.render(user).await
}
