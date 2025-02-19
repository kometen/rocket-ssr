use rocket::response::content::RawHtml;
use rocket::{async_trait, get, launch, routes};
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};

static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
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
    async fn render(&self) -> RawHtml<String>;
}

struct IndexPage;
#[async_trait]
impl IndexTemplate for IndexPage {
    async fn render(&self) -> RawHtml<String> {
        let favicon = FaviconPage.render().await;
        RawHtml(format!(
            "<DOCTYPE html><html><head>{favicon}</head><body><h1>Hello, world!</h1></body></html>",
        ))
    }
}

#[get("/")]
async fn index() -> RawHtml<String> {
    IndexPage.render().await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "static/favicon.ico",
        ))
        .mount("/", routes![index, favicon])
}
