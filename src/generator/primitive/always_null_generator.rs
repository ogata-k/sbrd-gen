use crate::builder::GeneratorBuilder;
use crate::error::{BuildError, GenerateError};
use crate::generator::{GeneratorBase, Randomizer};
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;

/// The generator with always generate [`DataValue::Null`]
///
/// [`DataValue::Null`]: ../../value/enum.DataValue.html#variant.Null
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AlwaysNullGenerator {}

impl<R: Randomizer + ?Sized> GeneratorBase<R> for AlwaysNullGenerator {
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
        _context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        Ok(DataValue::Null)
    }
}
