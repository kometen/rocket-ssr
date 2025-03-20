mod about_page;
mod api;
mod auth_context;
mod auth_handlers;
mod client;
mod index_page;
mod message_page;
mod models;
mod register_account_page;
mod request_guard;
mod session;
mod test_page;

use crate::about_page::about;
use crate::api::save_message;
use crate::client::PasswordlessClient;
use crate::index_page::index;
use crate::register_account_page::register_account;
use crate::test_page::test;

use auth_handlers::{login, logout, register};
use dotenv::dotenv;
use message_page::message;
use rocket::fs::FileServer;
use rocket::get;
use rocket::tokio::sync::RwLock;
use rocket::{launch, routes};
use rocket_dyn_templates::Template;
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};
use session::SessionStore;

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

    let session_store = RwLock::new(SessionStore::new());

    rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "static/favicon.ico",
        ))
        .mount("/static", FileServer::from("static"))
        .mount(
            "/",
            routes![
                index,
                message,
                save_message,
                about,
                favicon,
                favicon_static,
                register_account,
                test,
                logout
            ],
        )
        .mount("/passwordless/api", routes![register, login])
        .manage(client)
        .manage(session_store)
        .attach(Template::fairing())
}

#[cfg(test)]
mod tests;
