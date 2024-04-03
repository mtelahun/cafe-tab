use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct TabId(uuid::Uuid);

impl TabId {
    pub fn new() -> TabId {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for TabId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for TabId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
