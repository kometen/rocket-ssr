use rocket::fs::NamedFile;
use rocket::response::content::RawHtml;
use rocket::{async_trait, get, launch, routes};
use std::path::{Path, PathBuf};

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

#[get("/static/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, files])
}
