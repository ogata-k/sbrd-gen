use rand::Rng;

use crate::generators::error::{CompileError, GenerateError};
use crate::generators::Generator;
use crate::{DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct AlwaysNullGenerator {}

impl<R: Rng + ?Sized> Generator<R> for AlwaysNullGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let GeneratorBuilder { generator_type, .. } = builder;

        if generator_type != GeneratorType::AlwaysNull {
            return Err(CompileError::InvalidType(generator_type));
        }

        Ok(Self {})
    }

    fn is_nullable(&self) -> bool {
        true
    }

    fn generate_without_null(
        &self,
        _rng: &mut R,
        _value_map: &DataValueMap<String>,
    ) -> Result<DataValue, GenerateError> {
        Ok(DataValue::Null)
    }
}
