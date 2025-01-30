use serde::{Deserialize, Serialize};

use super::property::{self, Property};

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildSystem {
    pub(crate) name: String,
    pub(crate) version: Option<String>,
}

impl BuildSystem {
    #[must_use]
    pub fn with_version(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: Some(version.to_string()),
        }
    }
}

impl Property for BuildSystem {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: None,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }
}

property::impl_property_traits!(BuildSystem);
property::impl_try_from_tuple!(BuildSystem);
