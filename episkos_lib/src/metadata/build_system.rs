use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSystem {
    id: Uuid,
    pub(crate) name: String,
    pub(crate) version: String,
}

impl BuildSystem {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    pub fn with_id(mut self, id: Uuid) -> Self {
        self.id = id;
        self
    }
}
