use crate::generators::error::{CompileError, GenerateError};
use crate::generators::{Generator, RandomSelectableGenerator, Randomizer, WeightedSelectable};
use crate::{DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable};

pub struct RandomizeGenerator<R: 'static + Randomizer + ?Sized> {
    nullable: Nullable,
    selectable_values: Vec<WeightedSelectable<R>>,
}

impl<R: Randomizer + ?Sized> RandomSelectableGenerator<R> for RandomizeGenerator<R> {
    fn get_selectable(&self) -> &[WeightedSelectable<R>] {
        &self.selectable_values
    }
}

impl<R: Randomizer + ?Sized> Generator<R> for RandomizeGenerator<R> {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
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
            return Err(CompileError::InvalidType(generator_type));
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
        value_map: &DataValueMap,
    ) -> Result<DataValue, GenerateError> {
        self.choose(rng, value_map)
    }
}
