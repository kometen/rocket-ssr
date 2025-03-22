use crate::persistence::MessageRepository;
use crate::request_guard::LimitedId;
use rocket::get;
use rocket::http::Status;
use rocket::State;
use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub id: Option<String>,
    pub encrypted_message: String,
    pub iv: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub id: String,
    pub status: String,
}

#[post("/api/messages", format = "json", data = "<message>")]
pub async fn save_message(
    message: Json<EncryptedMessage>,
    repo: &State<MessageRepository>,
) -> Result<Json<MessageResponse>, Status> {
    println!("message: {:?}", message);

    match repo.save_message(&message).await {
        Ok(id) => Ok(Json(MessageResponse {
            id: id.to_string(),
            status: "success".to_string(),
        })),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/api/message/<id>")]
pub async fn get_message(
    id: LimitedId<'_>,
    repo: &State<MessageRepository>,
) -> Result<Json<EncryptedMessage>, Status> {
    match repo.get_message(id.0).await {
        Ok(Some(message)) => Ok(Json(message)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
