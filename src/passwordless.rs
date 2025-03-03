use crate::{
    client::PasswordlessClient,
    models::{RegisterRequest, SignInVerifyRequest},
};
use rocket::{http::Status, post, response::status::Custom, serde::json::Json, State};
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
) -> Result<Value, Custom<Value>> {
    match client.sign_in(&data).await {
        Ok(response) => Ok(json!(response)),
        Err(e) => Err(Custom(
            Status::InternalServerError,
            json!({ "error": e.to_string() }),
        )),
    }
}
