use rocket::{async_trait, get, launch, routes};
use rocket_dyn_templates::{context, Template};
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};

static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
    "/static/favicon.ico" => favicon_static => "favicon",
}

#[async_trait]
trait FaviconTemplate {
    async fn render(&self) -> String;
}

struct FaviconPage;
#[async_trait]
impl FaviconTemplate for FaviconPage {
    async fn render(&self) -> String {
        "<link rel='icon' type='image/x-icon' href='/static/favicon.ico'>".to_string()
    }
}

#[async_trait]
trait IndexTemplate {
    async fn render(&self) -> Template;
}

struct IndexPage;
#[async_trait]
impl IndexTemplate for IndexPage {
    async fn render(&self) -> Template {
        let favicon = FaviconPage.render().await;
        Template::render("index", context! {})
    }
}

#[get("/")]
async fn index() -> Template {
    IndexPage.render().await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "static/favicon.ico",
        ))
        .mount("/", routes![index, favicon, favicon_static])
        .attach(Template::fairing())
}
