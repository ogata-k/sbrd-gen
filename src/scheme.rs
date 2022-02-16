use crate::builder::ParentGeneratorBuilder;
use crate::error::{BuildError, IntoSbrdError, SchemeErrorKind, SchemeResult};
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap, ValueMap};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum ParserType {
    Yaml,
    Json,
}

impl Default for ParserType {
    fn default() -> Self {
        ParserType::Yaml
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SchemeBuilder {
    keys: Vec<String>,
    #[serde(rename = "generators")]
    builders: Vec<ParentGeneratorBuilder>,
}

impl SchemeBuilder {
    pub fn new(keys: Vec<String>, builders: Vec<ParentGeneratorBuilder>) -> SchemeBuilder {
        SchemeBuilder { keys, builders }
    }

    pub fn parse_from_str<T>(parse_type: ParserType, s: &str) -> SchemeResult<Self>
    where
        T: DeserializeOwned,
    {
        let parsed: Self = match parse_type {
            ParserType::Yaml => serde_yaml::from_str(s)
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::ParseError))?,
            ParserType::Json => serde_json::from_str(s)
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::ParseError))?,
        };

        Ok(parsed)
    }

    pub fn parse_from_reader<R, T>(parse_type: ParserType, rdr: R) -> SchemeResult<Self>
    where
        R: std::io::Read,
        T: DeserializeOwned,
    {
        let parsed: Self = match parse_type {
            ParserType::Yaml => serde_yaml::from_reader(rdr)
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::ParseError))?,
            ParserType::Json => serde_json::from_reader(rdr)
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::ParseError))?,
        };

        Ok(parsed)
    }

    pub fn build<R: 'static + Randomizer + ?Sized>(self) -> SchemeResult<Scheme<R>> {
        let SchemeBuilder {
            keys: specified_keys,
            builders,
        } = self;
        let mut generators = Vec::new();
        let mut checked = Vec::new();

        // check specified key's unique
        let mut cloned = specified_keys.clone();
        cloned.sort();
        cloned.dedup();
        if cloned.len() != specified_keys.len() {
            return Err(BuildError::SpecifiedKeyNotUnique(specified_keys)
                .into_sbrd_gen_error(SchemeErrorKind::BuildError));
        }

        for parent_builder in builders.into_iter() {
            let (key, builder) = parent_builder.split_key();

            if checked.contains(&key) {
                return Err(BuildError::AlreadyExistKey(key)
                    .into_sbrd_gen_error(SchemeErrorKind::BuildError));
            }

            let generator = builder.build()?;
            generators.push((key.clone(), generator));
            checked.push(key);
        }
        for specified_key in specified_keys.iter() {
            if !checked.contains(specified_key) {
                return Err(
                    BuildError::NotExistSpecifiedKey(specified_key.to_string(), checked)
                        .into_sbrd_gen_error(SchemeErrorKind::BuildError),
                );
            }
        }

        Ok(Scheme {
            keys: specified_keys,
            generators,
        })
    }
}

pub struct Scheme<R: 'static + Randomizer + ?Sized> {
    keys: Vec<String>,
    generators: Vec<(String, Box<dyn Generator<R>>)>,
}

impl<R: 'static + Randomizer + ?Sized> Scheme<R> {
    pub fn generate(&self, rng: &mut R) -> SchemeResult<GeneratedValues> {
        let mut generated_values = DataValueMap::new();
        for (key, generator) in self.generators.iter() {
            let generated = generator
                .generate(rng, &generated_values)
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::GenerateError))?;
            generated_values.insert(key, generated);
        }

        Ok(GeneratedValues {
            keys: self.get_keys(),
            generated_values,
        })
    }

    pub fn get_keys(&self) -> &[String] {
        &self.keys
    }
}

pub struct GeneratedValues<'a> {
    keys: &'a [String],
    generated_values: DataValueMap<&'a str>,
}

impl<'a> std::fmt::Debug for GeneratedValues<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_values = self.get_values_each_key();
        f.debug_map().entries(key_values).finish()
    }
}

impl<'a> std::fmt::Display for GeneratedValues<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_values = self.get_values_each_key();
        write!(f, "{{")?;
        for (i, (k, v)) in key_values.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            match v {
                DataValue::Int(v) => write!(f, "{}: {:?}", k, v)?,
                DataValue::Real(v) => write!(f, "{}: {:?}", k, v)?,
                DataValue::Bool(v) => write!(f, "{}: {:?}", k, v)?,
                DataValue::String(v) => write!(f, "{}: {:?}", k, v)?,
                DataValue::Null => write!(f, "{}: null", k)?,
            };
        }
        write!(f, "}}")
    }
}

impl<'a> GeneratedValues<'a> {
    pub fn get_all_values(&self) -> &DataValueMap<&str> {
        &self.generated_values
    }

    pub fn get_values_each_key<'b>(&'b self) -> Vec<(&'a str, &'b DataValue)> {
        let mut result = Vec::new();
        for key in self.keys.iter() {
            let value = self.generated_values.get(key.as_str()).unwrap_or_else(|| {
                panic!(
                    "Checked exist key \"{}\" is not exist generated values {:?}",
                    key,
                    self.generated_values
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect::<ValueMap<String, String>>()
                )
            });
            result.push((key.as_str(), value));
        }

        result
    }
}
