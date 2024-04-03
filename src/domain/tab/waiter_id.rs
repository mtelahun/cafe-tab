use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WaiterId(uuid::Uuid);

impl WaiterId {
    pub fn new() -> WaiterId {
        Self(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for WaiterId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for WaiterId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
