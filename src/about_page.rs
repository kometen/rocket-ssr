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
        let passwordless_api_key =
            &std::env::var("PASSWORDLESS_API_KEY").expect("PASSWORDLESS_API_KEY must be set.");
        let passwordless_api_url =
            &std::env::var("PASSWORDLESS_API_URL").expect("PASSWORDLESS_API_URL must be set.");
        Template::render(
            "about",
            context! { passwordless_api_url: passwordless_api_url, passwordless_api_key: passwordless_api_key  },
        )
    }
}

#[get("/about")]
pub async fn about() -> Template {
    AboutPage.render().await
}
