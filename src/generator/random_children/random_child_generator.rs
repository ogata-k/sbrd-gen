use crate::builder::GeneratorBuilder;
use crate::error::{BuildError, GenerateError};
use crate::generator::{GeneratorBase, Randomizer, WeightedChild, WeightedChildGeneratorBase};
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;

/// The generator with picked out the value from the input values or the value generated by picked out child generator
pub struct RandomChildGenerator<R: Randomizer + ?Sized> {
    nullable: bool,
    selectable_values: Vec<WeightedChild<R>>,
}

impl<R: Randomizer + ?Sized> WeightedChildGeneratorBase<R> for RandomChildGenerator<R> {
    fn get_selectable(&self) -> &[WeightedChild<R>] {
        &self.selectable_values
    }
}

impl<R: Randomizer + ?Sized> GeneratorBase<R> for RandomChildGenerator<R> {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            children,
            ..
        } = builder;

        if generator_type != GeneratorType::RandomChild {
            return Err(BuildError::InvalidType(generator_type));
        }

        let selectable_values = Self::build_selectable(children)?;

        Ok(Self {
            nullable,
            selectable_values,
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        self.generate_from_children(rng, context)
    }
}