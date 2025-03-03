use rocket::{async_trait, get};
use rocket_dyn_templates::{context, Template};

#[async_trait]
trait IndexTemplate {
    async fn render(&self) -> Template;
}

struct IndexPage;
#[async_trait]
impl IndexTemplate for IndexPage {
    async fn render(&self) -> Template {
        let passwordless_api_key =
            &std::env::var("PASSWORDLESS_API_KEY").expect("PASSWORDLESS_API_KEY must be set.");
        let passwordless_api_url =
            &std::env::var("PASSWORDLESS_API_URL").expect("PASSWORDLESS_API_URL must be set.");
        Template::render(
            "index",
            context! { passwordless_api_url: passwordless_api_url, passwordless_api_key: passwordless_api_key  },
        )
    }
}

#[get("/")]
pub async fn index() -> Template {
    IndexPage.render().await
}
