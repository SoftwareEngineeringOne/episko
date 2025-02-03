use crate::database::DatabaseObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::property::{self, Property};

#[derive(Debug, Serialize, Deserialize, DatabaseObject, FromRow, PartialOrd, Ord)]
#[db(table = "language")]
pub struct Language {
    #[serde(skip)]
    #[db(col = "id")]
    id: Vec<u8>,
    #[db(col = "name")]
    pub(crate) name: String,
    #[db(col = "version")]
    pub(crate) version: Option<String>,
}

impl Language {
    #[must_use]
    pub fn with_version(name: &str, version: &str) -> Self {
        let mut s = Language::new(name);
        s.version = Some(version.to_string());
        s.id = s.generate_id().into();
        s
    }
}

impl Property for Language {
    #[must_use]
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

property::impl_property_traits!(Language);
property::impl_try_from_tuple!(Language);
