use crate::session::UserProfile;
use rocket::serde::json::{json, Value};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;
use std::collections::HashMap;

pub struct AuthContext {
    pub user: Option<UserProfile>,
    pub passwordless_api_key: String,
    pub passwordless_api_url: String,
    custom_values: HashMap<String, Value>,
}

impl AuthContext {
    pub fn new(user: Option<UserProfile>) -> Self {
        let passwordless_api_key =
            std::env::var("PASSWORDLESS_API_KEY").expect("PASSWORDLESS_API_KEY must be set.");
        let passwordless_api_url =
            std::env::var("PASSWORDLESS_API_URL").expect("PASSWORDLESS_API_URL must be set.");

        Self {
            user,
            passwordless_api_key,
            passwordless_api_url,
            custom_values: HashMap::new(),
        }
    }

    pub fn insert<T: Serialize>(&mut self, key: &str, value: T) -> &mut Self {
        if let Ok(json_value) = serde_json::to_value(value) {
            self.custom_values.insert(key.to_string(), json_value);
        }
        self
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.custom_values.get(key)
    }

    pub fn render_template(&self, template_name: &str) -> Template {
        let template_name_owned = template_name.to_string();

        // Start with the default context values
        let mut context_map = serde_json::Map::new();

        // Add user info
        if let Ok(user_json) = serde_json::to_value(&self.user) {
            context_map.insert("user".to_string(), user_json);
        }

        // Add API credentials
        context_map.insert(
            "passwordless_api_url".to_string(),
            json!(self.passwordless_api_url),
        );
        context_map.insert(
            "passwordless_api_key".to_string(),
            json!(self.passwordless_api_key),
        );

        // Add all custom values
        for (key, value) in &self.custom_values {
            context_map.insert(key.clone(), value.clone());
        }

        // Convert to a Value
        let context = Value::Object(context_map);

        Template::render(template_name_owned, context)
    }
}
