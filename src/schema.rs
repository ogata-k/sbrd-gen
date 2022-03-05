#![deny(missing_debug_implementations)]
//! Module for schema

use crate::builder::ParentGeneratorBuilder;
use crate::error::{BuildError, GenerateError, IntoSbrdError, SchemaErrorKind, SchemaResult};
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap};
use serde::ser::Error;
use serde::{Deserialize, Serialize};

/// Builder for [`Schema`] is consisting of values at `keys` key that need to be output and builders at `generators` key
///
/// [`Schema`]: ./struct.Schema.html
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct SchemaBuilder {
    keys: Vec<String>,
    #[serde(rename = "generators")]
    builders: Vec<ParentGeneratorBuilder>,
}

impl SchemaBuilder {
    /// Constructor
    pub fn new(keys: Vec<String>, builders: Vec<ParentGeneratorBuilder>) -> SchemaBuilder {
        SchemaBuilder { keys, builders }
    }

    /// Build schema structure
    pub fn build<R: Randomizer + ?Sized>(self) -> SchemaResult<Schema<R>> {
        let SchemaBuilder {
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
                .into_sbrd_gen_error(SchemaErrorKind::BuildError));
        }

        for parent_builder in builders.into_iter() {
            let (key, builder) = parent_builder.split_key();

            if checked.contains(&key) {
                return Err(BuildError::AlreadyExistKey(key)
                    .into_sbrd_gen_error(SchemaErrorKind::BuildError));
            }

            let generator = builder.build()?;
            generators.push((key.clone(), generator));
            checked.push(key);
        }
        for specified_key in specified_keys.iter() {
            if !checked.contains(specified_key) {
                return Err(
                    BuildError::NotExistSpecifiedKey(specified_key.to_string(), checked)
                        .into_sbrd_gen_error(SchemaErrorKind::BuildError),
                );
            }
        }

        Ok(Schema {
            keys: specified_keys,
            generators,
        })
    }
}

/// Schema consisting of `keys` and` generators`
#[allow(missing_debug_implementations)]
pub struct Schema<R: Randomizer + ?Sized> {
    keys: Vec<String>,
    generators: Vec<(String, Box<dyn Generator<R>>)>,
}

impl<R: Randomizer + ?Sized> Schema<R> {
    /// Get specified keys
    pub fn get_keys(&self) -> &[String] {
        &self.keys
    }

    /// Generate a values set
    pub fn generate(&self, rng: &mut R) -> SchemaResult<GeneratedValues> {
        let mut generated_values = DataValueMap::new();
        for (key, generator) in self.generators.iter() {
            let generated = generator
                .generate(rng, &generated_values)
                .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::GenerateError))?;
            generated_values.insert(key, generated);
        }

        Ok(GeneratedValues {
            keys: self.get_keys(),
            generated_values,
        })
    }
}

/// Structure for generated values set
pub struct GeneratedValues<'a> {
    keys: &'a [String],
    generated_values: DataValueMap<&'a str>,
}

impl<'a> std::fmt::Debug for GeneratedValues<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_values = self
            .filter_values_with_key()
            .map_err(|e| std::fmt::Error::custom(e.to_string()))?;
        f.debug_map().entries(key_values).finish()
    }
}

impl<'a> std::fmt::Display for GeneratedValues<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key_values = self
            .filter_values_with_key()
            .map_err(|e| std::fmt::Error::custom(e.to_string()))?;
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
    /// Get all values regardless of whether the key is specified as the value for which output is required
    pub fn get_all_values(&self) -> &DataValueMap<&str> {
        &self.generated_values
    }

    /// Get all values for which the key is specified as the value for which output is required
    pub fn filter_values(&self) -> SchemaResult<Vec<&DataValue>> {
        let mut result = Vec::new();
        for key in self.keys.iter() {
            let value_result = self.generated_values.get(key.as_str());
            let value = value_result.ok_or_else(|| {
                GenerateError::NotExistGeneratedKey(
                    key.to_string(),
                    self.generated_values
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.clone()))
                        .collect::<DataValueMap<String>>(),
                )
                .into_sbrd_gen_error(SchemaErrorKind::GenerateError)
            })?;

            result.push(value);
        }

        Ok(result)
    }

    /// Get all keys and values for which the key is specified as the value for which output is required
    pub fn filter_values_with_key<'b>(&'b self) -> SchemaResult<Vec<(&'a str, &'b DataValue)>> {
        let mut result = Vec::new();
        for key in self.keys.iter() {
            let value_result = self.generated_values.get(key.as_str());
            let value = value_result.ok_or_else(|| {
                GenerateError::NotExistGeneratedKey(
                    key.to_string(),
                    self.generated_values
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.clone()))
                        .collect::<DataValueMap<String>>(),
                )
                .into_sbrd_gen_error(SchemaErrorKind::GenerateError)
            })?;

            result.push((key.as_str(), value));
        }

        Ok(result)
    }

    /// Convert to a sequence from all values for which the key is specified as the value for which output is required
    pub fn into_values(self) -> SchemaResult<Vec<DataValue>> {
        let mut result = Vec::new();
        let GeneratedValues {
            keys,
            mut generated_values,
        } = self;

        // check
        for key in keys.iter() {
            if !generated_values.contains_key(key.as_str()) {
                return Err(GenerateError::NotExistGeneratedKey(
                    key.to_string(),
                    generated_values
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect::<DataValueMap<String>>(),
                )
                .into_sbrd_gen_error(SchemaErrorKind::GenerateError));
            }
        }

        // drain
        for key in keys.iter() {
            let value_result = generated_values.remove_entry(key.as_str());
            let (_, value) = value_result
                .unwrap_or_else(|| panic!("Already checked {}'s value is not exist.", key));

            result.push(value);
        }

        Ok(result)
    }

    /// Convert to a sequence from all keys and values for which the key is specified as the value for which output is required
    pub fn into_values_with_key(self) -> SchemaResult<Vec<(String, DataValue)>> {
        let mut result = Vec::new();
        let GeneratedValues {
            keys,
            mut generated_values,
        } = self;

        // check
        for key in keys.iter() {
            if !generated_values.contains_key(key.as_str()) {
                return Err(GenerateError::NotExistGeneratedKey(
                    key.to_string(),
                    generated_values
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect::<DataValueMap<String>>(),
                )
                .into_sbrd_gen_error(SchemaErrorKind::GenerateError));
            }
        }

        // drain
        for key in keys.iter() {
            let value_result = generated_values.remove_entry(key.as_str());
            let (key, value) = value_result
                .unwrap_or_else(|| panic!("Already checked {}'s value is not exist.", key));

            result.push((key.to_string(), value));
        }

        Ok(result)
    }
}
