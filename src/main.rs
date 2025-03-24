mod api;
mod auth;
mod models;
mod persistence;
mod routes;

use crate::auth::client::PasswordlessClient;
use crate::routes::test_page::test;

use api::{get_message, login, logout, register, save_message};
use auth::session;
use dotenv::dotenv;
use rocket::{fs::FileServer, get, routes, tokio::sync::RwLock};
use rocket_dyn_templates::Template;
use rocket_include_static_resources::{static_resources_initializer, static_response_handler};
use routes::{about, index, message, register_account, view_message};
use session::SessionStore;
use std::fs;
use std::path::Path;

static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
    "/static/favicon.ico" => favicon_static => "favicon",
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client = PasswordlessClient::new(
        &std::env::var("PASSWORDLESS_API_SECRET").expect("PASSWORDLESS_API_SECRET must be set."),
        &std::env::var("PASSWORDLESS_API_URL").expect("PASSWORDLESS_API_URL must be set."),
    );

    let session_store = RwLock::new(SessionStore::new());

    // Create a data directory if it doesn't exist
    let data_dir = Path::new("./data");
    if !data_dir.exists() {
        fs::create_dir(data_dir)?;
    }

    // Create a database file if it doesn't exist
    let db_path = data_dir.join("sqlite.db");
    if !db_path.exists() {
        fs::File::create(&db_path)?;
    }

    let db_url = format!("sqlite://{}", db_path.display());

    let db = persistence::MessageRepository::new(&db_url).await?;

    let rocket = rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "static/favicon.ico",
        ))
        .mount("/static", FileServer::from("static"))
        .mount(
            "/",
            routes![
                index,
                message,
                get_message,
                save_message,
                view_message,
                test,
                about,
                favicon,
                favicon_static,
                register_account,
                logout
            ],
        )
        .mount("/passwordless/api", routes![register, login])
        .manage(client)
        .manage(session_store)
        .manage(db)
        .attach(Template::fairing());

    rocket.launch().await?;

    Ok(())
}
