use crate::builder::GeneratorBuilder;
use crate::error::{BuildError, GenerateError};
use crate::generator::{CasedChild, CasedChildGeneratorBase, GeneratorBase, Randomizer};
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;

/// The generator with use the child generator which satisfy condition.
/// If a child generator's condition is [`Option::Some`], then evaluate it's condition.
/// If a child generator's condition is [`Option::None`], then default condition. Default condition always must exist.
///
/// [`Option::Some`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.Some
/// [`Option::None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
pub struct CaseWhenGenerator<R: Randomizer + ?Sized> {
    nullable: bool,
    children: Vec<CasedChild<R>>,
}

impl<R: Randomizer + ?Sized> CasedChildGeneratorBase<R> for CaseWhenGenerator<R> {
    fn get_children(&self) -> &[CasedChild<R>] {
        self.children.as_slice()
    }
}

impl<R: Randomizer + ?Sized> GeneratorBase<R> for CaseWhenGenerator<R> {
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
