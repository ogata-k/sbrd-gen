use crate::builder::{GeneratorBuilder, Nullable};
use crate::error::{BuildError, GenerateError};
use crate::generator::{Generator, RandomValueChildGenerator, Randomizer, WeightedValueChild};
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;

pub struct RandomizeGenerator<R: Randomizer + ?Sized> {
    nullable: Nullable,
    selectable_values: Vec<WeightedValueChild<R>>,
}

impl<R: Randomizer + ?Sized> RandomValueChildGenerator<R> for RandomizeGenerator<R> {
    fn get_selectable(&self) -> &[WeightedValueChild<R>] {
        &self.selectable_values
    }
}

impl<R: Randomizer + ?Sized> Generator<R> for RandomizeGenerator<R> {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            children,
            chars,
            values,
            filepath,
            ..
        } = builder;

        if generator_type != GeneratorType::Randomize {
            return Err(BuildError::InvalidType(generator_type));
        }

        let selectable_values = Self::build_selectable(children, chars, values, filepath)?;

        Ok(Self {
            nullable,
            selectable_values,
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        value_map: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        self.choose(rng, value_map)
    }
}
