//! # Advanced Property trait and macro
use serde::{de::DeserializeOwned, Serialize};
use sha2::{Digest, Sha256};
use std::hash::Hash;

/// The Property trait is implemented by advanced properties.
///
/// It is used to allow for comparison of properties, independent of
/// their case.
/// ```
/// use episkos_lib::metadata::Category;
/// use episkos_lib::metadata::property::Property as _;
///
/// let category1 = Category::new("Example");
/// let category2 = Category::new("example");
///
/// category1 == category2; // -> true
///
/// // Fields of advanced properties are converted to lowercase
/// // for comparisons and id generation
/// category1.canonical(); // -> "example"
/// ```
pub trait Property: Serialize + DeserializeOwned + PartialEq + Eq + Hash {
    fn name(&self) -> &str;
    fn version(&self) -> Option<&str> {
        None
    }
    fn canonical(&self) -> String {
        format!(
            "{}{}",
            self.name().to_lowercase(),
            self.version().unwrap_or_default().to_lowercase()
        )
    }
    fn generate_id(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(self.canonical());
        hasher.finalize().into()
    }
}

macro_rules! impl_property_traits {
    ($type:ty) => {
        impl ::std::cmp::PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                self.canonical() == other.canonical()
            }
        }

        impl ::std::cmp::Eq for $type {}

        impl ::std::hash::Hash for $type {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.canonical().hash(state)
            }
        }
    };
}

pub(crate) use impl_property_traits;
