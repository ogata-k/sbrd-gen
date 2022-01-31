use serde::Serialize;

use crate::builder::GeneratorBuilder;

#[derive(Serialize, Debug, PartialEq, Clone)]
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
