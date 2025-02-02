use crate::database::object::DatabaseObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use super::property::{self, Property};

#[derive(Debug, Serialize, Deserialize, DatabaseObject, FromRow)]
#[db(table = "ide")]
pub struct Ide {
    #[db(col = "id")]
    id: Vec<u8>,
    #[db(col = "name")]
    pub(crate) name: String,
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
}

property::impl_property_traits!(Ide);
