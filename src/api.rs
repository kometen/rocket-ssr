use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct EncryptedMessage {
    pub encrypted_message: String,
    pub iv: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub id: String,
    pub status: String,
}

#[post("/api/messages", format = "json", data = "<message>")]
pub fn save_message(message: Json<EncryptedMessage>) -> Json<MessageResponse> {
    println!(
        "Encrypted message: {}, iv: {}",
        message.encrypted_message, message.iv
    );

    // Store the encrypted message and IV in your database
    // In a real application, you'd generate a unique ID and save the data

    let message_id = format!("{}", rand::random::<u64>());

    // For demonstration, we're just returning an ID without actually saving
    Json(MessageResponse {
        id: message_id,
        status: "success".to_string(),
    })
}
