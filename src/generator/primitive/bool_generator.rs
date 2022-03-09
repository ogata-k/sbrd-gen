use crate::builder::{GeneratorBuilder, Nullable};
use crate::error::{BuildError, GenerateError};
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;

/// The generator with generate [`DataValue::Bool`] value
///
/// [`DataValue::Bool`]: ../../value/enum.DataValue.html#variant.Bool
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BoolGenerator {
    nullable: Nullable,
}

impl<R: Randomizer + ?Sized> Generator<R> for BoolGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            ..
        } = builder;

        if generator_type != GeneratorType::Bool {
            return Err(BuildError::InvalidType(generator_type));
        }

        Ok(Self { nullable })
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        _context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        Ok(DataValue::Bool(rng.gen_bool(0.5)))
    }
}
