mod about_page;
mod auth_context;
mod client;
mod index_page;
mod models;
mod passwordless;
mod register_account_page;
mod test_page;

use crate::about_page::about;
use crate::client::PasswordlessClient;
use crate::index_page::index;
use crate::register_account_page::register_account;
use crate::test_page::test;
use dotenv::dotenv;

use passwordless::{login, register};
use rocket::fs::FileServer;
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
    dotenv().ok();

    let client = PasswordlessClient::new(
        &std::env::var("PASSWORDLESS_API_SECRET").expect("PASSWORDLESS_API_SECRET must be set."),
        &std::env::var("PASSWORDLESS_API_URL").expect("PASSWORDLESS_API_URL must be set."),
    );

    rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "static/favicon.ico",
        ))
        .mount("/static", FileServer::from("static"))
        .mount(
            "/",
            routes![
                index,
                about,
                favicon,
                favicon_static,
                register_account,
                test
            ],
        )
        .mount("/passwordless/api", routes![register, login])
        .manage(client)
        .attach(Template::fairing())
}

#[cfg(test)]
mod tests;
