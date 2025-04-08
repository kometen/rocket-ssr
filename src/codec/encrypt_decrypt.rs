pub mod crypto {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    use anyhow::Result;
    use base64::{engine::general_purpose::STANDARD as Base64, Engine};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct EncryptedMessage {
        pub encrypted_message: String,
        pub iv: String,
    }

    pub fn encrypt_message(message: &str, key_base64: &str) -> Result<EncryptedMessage> {
        let key_bytes = Base64
            .decode(key_base64)
            .map_err(|e| anyhow::anyhow!("Failed to decode base64 key: {}", e))?;

        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(|e| anyhow::anyhow!("Invalid key length: {}", e))?;

        let iv = Nonce::default();
        let encrypted_bytes = cipher
            .encrypt(&iv, message.as_bytes())
            .map_err(|e| anyhow::anyhow!("Failed to encrypt message: {}", e))?;

        let encrypted_message = Base64.encode(&encrypted_bytes);
        let iv = Base64.encode(&iv);

        Ok(EncryptedMessage {
            encrypted_message,
            iv,
        })
    }

    pub fn decrypt_message(message: &EncryptedMessage, key_base64: &str) -> Result<String> {
        let key_bytes = Base64
            .decode(key_base64)
            .map_err(|e| anyhow::anyhow!("Failed to decode base64 key: {}", e))?;

        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

        let iv_decoded = Base64
            .decode(&message.iv)
            .map_err(|e| anyhow::anyhow!("Failed to decode base64 iv: {}", e))?;
        let nonce = Nonce::from_slice(&iv_decoded);

        let encrypted_bytes = Base64
            .decode(&message.encrypted_message)
            .map_err(|e| anyhow::anyhow!("Failed to decode base64 encrypted message: {}", e))?;

        let decrypted_bytes = cipher
            .decrypt(nonce, encrypted_bytes.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to decrypt message: {}", e))?;

        let decrypted_text = String::from_utf8(decrypted_bytes).map_err(|e| {
            anyhow::anyhow!("Failed to convert decrypted bytes to UTF-8 string: {}", e)
        })?;

        Ok(decrypted_text)
    }
}
