use crate::builder::GeneratorBuilder;
use crate::error::{BuildError, GenerateError};
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;

/// The generator with always generate null
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AlwaysNullGenerator {}

impl<R: Randomizer + ?Sized> Generator<R> for AlwaysNullGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
    where
        Self: Sized,
    {
        let GeneratorBuilder { generator_type, .. } = builder;

        if generator_type != GeneratorType::AlwaysNull {
            return Err(BuildError::InvalidType(generator_type));
        }

        Ok(Self {})
    }

    fn is_nullable(&self) -> bool {
        true
    }

    fn generate_without_null(
        &self,
        _rng: &mut R,
        _value_map: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        Ok(DataValue::Null)
    }
}
