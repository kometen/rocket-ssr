mod index_page;
mod test_page;

use crate::index_page::index;
use crate::test_page::test;

use rocket::get;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};

static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
    "/static/favicon.ico" => favicon_static => "favicon",
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "static/favicon.ico",
        ))
        .mount("/", routes![index, favicon, favicon_static, test])
        .attach(Template::fairing())
}

#[cfg(test)]
mod tests;
