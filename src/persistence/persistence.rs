use crate::api::api::EncryptedMessage;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::{rng, RngCore};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};

pub struct MessageRepository {
    pool: SqlitePool,
}

impl MessageRepository {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await?;

        // Create table if it doesn't exist
        let create_table_result = sqlx::query(
            "CREATE TABLE IF NOT EXISTS messages (
                    id TEXT PRIMARY KEY NOT NULL,
                    encrypted_message TEXT NOT NULL,
                    iv TEXT NOT NULL,
                    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                )",
        )
        .execute(&db_pool)
        .await;

        match create_table_result {
            Ok(_) => println!("Table present or created successfully"),
            Err(e) => println!("Error creating table: {}", e),
        }

        Ok(Self { pool: db_pool })
    }

    fn generate_random_id(length_bytes: usize) -> String {
        let mut bytes = vec![0u8; length_bytes];
        rng().fill_bytes(&mut bytes);
        URL_SAFE_NO_PAD.encode(&bytes)
    }

    pub async fn save_message(&self, message: &EncryptedMessage) -> Result<String, sqlx::Error> {
        let random_id = Self::generate_random_id(16);
        let now = chrono::Utc::now();

        let sql =
            "INSERT INTO messages (id, encrypted_message, iv, created_at) VALUES (?, ?, ?, ?)";
        sqlx::query(sql)
            .bind(&random_id)
            .bind(&message.encrypted_message)
            .bind(&message.iv)
            .bind(&now.to_rfc3339())
            .execute(&self.pool)
            .await?;

        Ok(random_id)
    }

    pub async fn get_message(&self, id: &str) -> Result<Option<EncryptedMessage>, sqlx::Error> {
        let sql = "SELECT id, encrypted_message, iv FROM messages WHERE id = ?";
        let row = sqlx::query(sql).bind(id).fetch_optional(&self.pool).await?;

        match row {
            Some(row) => {
                let id = row.try_get("id")?;
                let encrypted_message = row.try_get("encrypted_message")?;
                let iv = row.try_get("iv")?;

                Ok(Some(EncryptedMessage {
                    id,
                    encrypted_message,
                    iv,
                }))
            }
            None => Ok(None),
        }
    }
}
