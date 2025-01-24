use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub(crate) id: Option<i32>,
    pub(crate) name: String,
    pub(crate) version: String,
}

impl Language {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            id: None,
            name: name.to_string(),
            version: version.to_string(),
        }
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }
}
