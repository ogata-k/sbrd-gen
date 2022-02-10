use rand::Rng;

use crate::generators::error::{CompileError, GenerateError};
use crate::generators::Generator;
use crate::{DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct BoolGenerator {
    key: Option<String>,
    condition: Option<String>,
    nullable: Nullable,
}

impl<R: Rng + ?Sized> Generator<R> for BoolGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            key,
            condition,
            ..
        } = builder;

        if generator_type != GeneratorType::Bool {
            return Err(CompileError::InvalidType(generator_type));
        }

        Ok(Self {
            key,
            condition,
            nullable,
        })
    }

    fn get_key(&self) -> Option<&str> {
        self.key.as_ref().map(|s| s.as_ref())
    }

    fn get_condition(&self) -> Option<&str> {
        self.condition.as_ref().map(|s| s.as_ref())
    }

    fn get_nullable(&self) -> &Nullable {
        &self.nullable
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        _value_map: &DataValueMap<String>,
    ) -> Result<DataValue, GenerateError> {
        Ok(DataValue::Bool(rng.gen_bool(0.5)))
    }
}
