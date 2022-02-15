use crate::builder::{GeneratorBuilder, Nullable};
use crate::generator::error::{CompileError, GenerateError};
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct BoolGenerator {
    nullable: Nullable,
}

impl<R: Randomizer + ?Sized> Generator<R> for BoolGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            ..
        } = builder;

        if generator_type != GeneratorType::Bool {
            return Err(CompileError::InvalidType(generator_type));
        }

        Ok(Self { nullable })
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        _value_map: &DataValueMap,
    ) -> Result<DataValue, GenerateError> {
        Ok(DataValue::Bool(rng.gen_bool(0.5)))
    }
}
