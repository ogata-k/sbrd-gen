use crate::builder::{GeneratorBuilder, Nullable};
use crate::error::{BuildError, GenerateError};
use crate::generator::{CasedChild, Generator, RandomCasedChildGenerator, Randomizer};
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;

/// The generator with use the child generator which satisfy condition.
/// If a child generator's condition is Option::Some, then evaluate it's condition.
/// If a child generator's condition is None, then default condition. Default condition always must exist.
pub struct CaseWhenGenerator<R: Randomizer + ?Sized> {
    nullable: Nullable,
    children: Vec<CasedChild<R>>,
}

impl<R: Randomizer + ?Sized> RandomCasedChildGenerator<R> for CaseWhenGenerator<R> {
    fn get_children(&self) -> &[CasedChild<R>] {
        self.children.as_slice()
    }
}

impl<R: Randomizer + ?Sized> Generator<R> for CaseWhenGenerator<R> {
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

        if generator_type != GeneratorType::CaseWhen {
            return Err(BuildError::InvalidType(generator_type));
        }

        let _children = Self::build_selectable(children)?;

        Ok(Self {
            nullable,
            children: _children,
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
        self.generate_from_children(rng, value_map)
    }
}
