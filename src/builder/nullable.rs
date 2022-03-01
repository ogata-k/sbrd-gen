//! Module for nullable

use serde::{Deserialize, Serialize};

/// Nullable option
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Nullable {
    use_null: bool,
}

impl Nullable {
    /// Create as required
    pub fn new_required() -> Self {
        Self { use_null: false }
    }

    /// Create as nullable
    pub fn new_nullable() -> Self {
        Self { use_null: true }
    }

    /// Is required?
    pub fn is_required(&self) -> bool {
        !self.use_null
    }

    /// Is nullable?
    pub fn is_nullable(&self) -> bool {
        self.use_null
    }
}
