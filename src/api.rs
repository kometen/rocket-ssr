use crate::persistence::EncryptedMessage;
use crate::persistence::MessageRepository;
use rocket::get;
use rocket::http::Status;
use rocket::State;
use rocket::{post, serde::json::Json};
use serde::Serialize;

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
    match repo.save_message(&message).await {
        Ok(id) => Ok(Json(MessageResponse {
            id: id.to_string(),
            status: "success".to_string(),
        })),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/api/messages/<id>")]
pub async fn get_message(
    id: String,
    repo: &State<MessageRepository>,
) -> Result<Json<EncryptedMessage>, Status> {
    match repo.get_message(&id).await {
        Ok(Some(message)) => Ok(Json(message)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}
