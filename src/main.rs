use rocket::response::content;
use rocket::{async_trait, get, launch, routes};

#[async_trait]
trait IndexTemplate {
    async fn render(&self) -> content::RawHtml<String>;
}

struct IndexPage;
#[async_trait]
impl IndexTemplate for IndexPage {
    async fn render(&self) -> content::RawHtml<String> {
        content::RawHtml(format!("<h1>Hello, world!</h1>"))
    }
}

#[get("/")]
async fn index() -> content::RawHtml<String> {
    IndexPage.render().await
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
