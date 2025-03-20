use crate::database::DatabaseObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::property::{self, Property};

#[derive(Debug, Serialize, Deserialize, DatabaseObject, FromRow, PartialOrd, Ord)]
#[db(table = "ide")]
pub struct Ide {
    #[serde(skip)]
    #[db(col = "id")]
    id: Vec<u8>,
    #[db(col = "name")]
    pub name: String,
}

impl Property for Ide {
    fn new(name: &str) -> Self {
        let mut s = Self {
            id: vec![],
            name: name.to_string(),
        };
        s.id = s.generate_id().to_vec();
        s
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn update_id(&mut self) {
        self.id = self.generate_id().to_vec();
    }
}

property::impl_property_traits!(Ide);
