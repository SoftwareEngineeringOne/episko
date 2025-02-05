use serde::{Deserialize, Serialize};

use super::property::{self, Property};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ide {
    pub(crate) name: String,
}

impl Property for Ide {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
}

property::impl_property_traits!(Ide);
