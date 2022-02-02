use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub struct Nullable{
    use_null: bool,
}

impl Nullable {
    pub fn new_as_required() -> Self
    {
        Self{
            use_null: true,
        }
    }

    pub fn new_as_nullable() -> Self
    {
        Self{
            use_null: false,
        }
    }

    pub fn is_required(&self) -> bool
    {
        !self.use_null
    }

    pub fn is_nullable(&self) -> bool
    {
        self.use_null
    }
}