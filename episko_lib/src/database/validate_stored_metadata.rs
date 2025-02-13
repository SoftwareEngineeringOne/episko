use crate::metadata::Metadata;

use super::Result;

impl Metadata {
    /// Validate the hash of a given metadata and update
    /// the cache if not up to date.
    ///
    /// TODO!
    pub async fn validate_db(&self) -> Result<bool> {
        todo!()
    }
}
