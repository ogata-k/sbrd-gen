use crate::WithKeyBuilder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Scheme {
    keys: Vec<String>,
    #[serde(rename = "generators")]
    builders: Vec<WithKeyBuilder>,
}

impl Scheme {
    pub fn new(keys: Vec<String>, builders: Vec<WithKeyBuilder>) -> Scheme {
        Scheme { keys, builders }
    }
}
