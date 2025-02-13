use crate::database::DatabaseObject;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use super::property::{self, Property};

#[derive(Debug, Serialize, Deserialize, DatabaseObject, FromRow, PartialOrd, Ord)]
#[db(table = "build_system")]
pub struct BuildSystem {
    #[serde(skip)]
    #[db(col = "id")]
    id: Vec<u8>,
    #[db(col = "name")]
    pub(crate) name: String,
    #[db(col = "version")]
    pub(crate) version: Option<String>,
}

impl BuildSystem {
    #[must_use]
    pub fn with_version(name: &str, version: &str) -> Self {
        let mut s = BuildSystem::new(name);
        s.version = Some(version.to_string());
        s.id = s.generate_id().into();
        s
    }
}

impl Property for BuildSystem {
    fn new(name: &str) -> Self {
        let mut s = Self {
            id: vec![],
            name: name.to_string(),
            version: None,
        };
        s.update_id();
        s
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    fn update_id(&mut self) {
        self.id = self.generate_id().to_vec();
    }
}

property::impl_property_traits!(BuildSystem);
property::impl_try_from_tuple!(BuildSystem);
