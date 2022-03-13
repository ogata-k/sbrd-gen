use crate::builder::{GeneratorBuilder, Nullable};
use crate::error::{BuildError, GenerateError};
use crate::generator::{GeneratorBase, Randomizer, SingleOptionValueGeneratorBase};
use crate::value::{DataValue, DataValueMap, SbrdInt};
use crate::GeneratorType;

/// The generator that get available index  of 0-index for value from the values.
pub struct GetValueIndexGenerator {
    nullable: Nullable,
    values_count: usize,
}

impl<R: Randomizer + ?Sized> SingleOptionValueGeneratorBase<R, ()> for GetValueIndexGenerator {
    fn parse(_input: &str) -> Result<(), BuildError> {
        Ok(())
    }
}

impl<R: Randomizer + ?Sized> GeneratorBase<R> for GetValueIndexGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            chars,
            values,
            filepath,
            ..
        } = builder;

        if generator_type != GeneratorType::GetValueIndex {
            return Err(BuildError::InvalidType(generator_type));
        }

        let selectable_values =
            <Self as SingleOptionValueGeneratorBase<R, ()>>::build_selectable(chars, values, filepath)?;

        Ok(Self {
            nullable,
            values_count: selectable_values.len(),
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        _context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        let gen_value = rng.gen_range(0..self.values_count);
        if gen_value <= SbrdInt::MAX as usize {
            Ok(DataValue::Int(gen_value as SbrdInt))
        } else {
            Err(GenerateError::FailGenerate(
                "Generated value is too big".to_string(),
            ))
        }
    }
}
