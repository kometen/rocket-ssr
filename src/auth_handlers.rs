use crate::{
    client::PasswordlessClient,
    models::{RegisterRequest, SignInVerifyRequest},
    session::{SessionStore, UserProfile},
};
use rocket::{
    get,
    http::{Cookie, Status},
    post,
    response::status::Custom,
    serde::json::Json,
    tokio::sync::RwLock,
    State,
};
use rocket::{http::CookieJar, response::Redirect};
use serde_json::{json, Value};

fn extract_string_field(data: &Value, field: &str) -> Result<String, Custom<Value>> {
    match data.get(field) {
        Some(val) => match val.as_str() {
            Some(s) => Ok(s.to_string()),
            None => Err(Custom(
                Status::BadRequest,
                json!({"error": format!("{} must be a string", field)}),
            )),
        },
        None => Err(Custom(
            Status::BadRequest,
            json!({"error": format!("{} is required", field)}),
        )),
    }
}

#[post("/register", format = "json", data = "<data>")]
pub async fn register(
    client: &State<PasswordlessClient>,
    data: Json<Value>,
) -> Result<Value, Custom<Value>> {
    let user_id = extract_string_field(&data, "userId")?;
    let username = extract_string_field(&data, "username")?;
    let display_name = extract_string_field(&data, "alias")?;

    let register_options = RegisterRequest {
        user_id,
        username,
        display_name,
    };

    println!(
        "user_id: {}, username: {}, display_name: {}",
        register_options.user_id, register_options.username, register_options.display_name
    );

    match client.register_token(&register_options).await {
        Ok(token) => Ok(json!(token)),
        Err(e) => Err(Custom(
            Status::InternalServerError,
            json!({ "error": e.to_string() }),
        )),
    }
}

#[post("/login", format = "json", data = "<data>")]
pub async fn login(
    client: &State<PasswordlessClient>,
    data: Json<SignInVerifyRequest>,
    cookies: &CookieJar<'_>,
    session_store: &State<RwLock<SessionStore>>,
) -> Result<Value, Custom<Value>> {
    match client.sign_in(&data).await {
        Ok(response) => {
            if let Some(user_id) = extract_user_id_from_response(&response) {
                let session_token = uuid::Uuid::new_v4().to_string();

                let profile = UserProfile {
                    user_id,
                    username: extract_username_from_response(&response),
                    display_name: extract_display_name_from_response(&response),
                };

                let mut store = session_store.write().await;
                store.add_session(session_token.clone(), profile);

                cookies.add(Cookie::new("session_token", session_token));

                Ok(json!({"success": true, "message": "Login successful"}))
            } else {
                Err(Custom(
                    Status::InternalServerError,
                    json!({ "error": "Failed to extract user information" }),
                ))
            }
        }
        Err(e) => Err(Custom(
            Status::InternalServerError,
            json!({ "error": e.to_string() }),
        )),
    }
}

#[get("/logout")]
pub async fn logout(
    cookies: &CookieJar<'_>,
    session_store: &State<RwLock<SessionStore>>,
) -> Redirect {
    if let Some(token) = cookies.get("session_token") {
        let token_val = token.value().to_string();
        let mut store = session_store.write().await;
        store.remove_session(&token_val);
        cookies.remove(Cookie::from("session_token"));
    }
    Redirect::to("/")
}

fn extract_user_id_from_response(response: &Value) -> Option<String> {
    response
        .get("user_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn extract_username_from_response(response: &Value) -> Option<String> {
    response
        .get("username")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

fn extract_display_name_from_response(response: &Value) -> Option<String> {
    response
        .get("display_name")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}
