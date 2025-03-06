use crate::auth_context::AuthContext;
use rocket::{async_trait, get};
use rocket_dyn_templates::Template;

#[async_trait]
trait RegisterAccountTemplate {
    async fn render(&self) -> Template;
}

struct RegisterAccountPage;
#[async_trait]
impl RegisterAccountTemplate for RegisterAccountPage {
    async fn render(&self) -> Template {
        AuthContext::new().render_template("register_account")
    }
}

#[get("/register")]
pub async fn register_account() -> Template {
    RegisterAccountPage.render().await
}
