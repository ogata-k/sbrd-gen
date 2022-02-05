use serde::{Deserialize, Serialize};

use crate::builder::GeneratorBuilder;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Scheme {
    keys: Vec<String>,
    generators: Vec<GeneratorBuilder>,
}

impl Scheme {
    pub fn new(keys: Vec<String>, builders: Vec<GeneratorBuilder>) -> Scheme {
        Scheme {
            keys: keys,
            generators: builders,
        }
    }
}
