use crate::builder::ParentGeneratorBuilder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Scheme {
    keys: Vec<String>,
    #[serde(rename = "generators")]
    builders: Vec<ParentGeneratorBuilder>,
}

impl Scheme {
    pub fn new(keys: Vec<String>, builders: Vec<ParentGeneratorBuilder>) -> Scheme {
        Scheme { keys, builders }
    }
}
