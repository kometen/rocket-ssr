#[cfg(test)]
mod tests {
    use crate::codec::encrypt_decrypt::crypto::{decrypt_message, encrypt_message};
    use base64::{engine::general_purpose::STANDARD as Base64, Engine};

    #[test]
    fn test_encrypt_decrypt() {
        let plaintext = r#"If you can look into the seeds of time,
        And say which grain will grow and which will not, Speak then to me.

        Shakespeare"#;

        let key_bytes = rand::random::<[u8; 32]>();
        let key_base64 = Base64.encode(&key_bytes);
        let ciphertext = encrypt_message(plaintext, &key_base64).unwrap();
        let decrypted_text = decrypt_message(&ciphertext, &key_base64).unwrap();
        assert_eq!(plaintext, decrypted_text);
    }
}
