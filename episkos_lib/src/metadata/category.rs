use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub(crate) id: Option<i32>,
    pub(crate) name: String,
}

impl Category {
    pub fn new(name: &str) -> Self {
        Self {
            id: None,
            name: name.to_string(),
        }
    }

    pub fn with_id(mut self, id: i32) -> Self {
        self.id = Some(id);
        self
    }
}
