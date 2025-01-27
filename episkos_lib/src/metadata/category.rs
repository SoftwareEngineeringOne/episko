use serde::{Deserialize, Serialize};

use super::property::{self, Property};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub(crate) name: String,
}

impl Category {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Property for Category {
    fn name(&self) -> &str {
        &self.name
    }
}

property::impl_property_traits!(Category);
