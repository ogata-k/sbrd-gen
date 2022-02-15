use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct Nullable {
    use_null: bool,
}

impl Nullable {
    pub fn new_required() -> Self {
        Self { use_null: false }
    }

    pub fn new_nullable() -> Self {
        Self { use_null: true }
    }

    pub fn is_required(&self) -> bool {
        !self.use_null
    }

    pub fn is_nullable(&self) -> bool {
        self.use_null
    }
}
