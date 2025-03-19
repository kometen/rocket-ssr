use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub username: Option<String>,
    pub aliases: Option<Vec<String>>,
}

pub(crate) struct SessionStore {
    sessions: HashMap<String, UserProfile>,
}

impl SessionStore {
    pub fn new() -> Self {
        SessionStore {
            sessions: HashMap::new(),
        }
    }

    pub fn add_session(&mut self, token: String, profile: UserProfile) {
        self.sessions.insert(token, profile);
    }

    pub fn get_profile(&self, token: &str) -> Option<UserProfile> {
        self.sessions.get(token).cloned()
    }

    pub fn remove_session(&mut self, token: &str) {
        self.sessions.remove(token);
    }
}
