use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TodoEntry {
    pub message: String,
    pub active: bool,
}

impl TodoEntry {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            active: true,
        }
    }
}
