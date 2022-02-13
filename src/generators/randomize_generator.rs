use crate::generators::error::{CompileError, GenerateError};
use crate::generators::{Generator, Randomizer};
use crate::{
    ChildGeneratorBuilder, DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable,
    Weight,
};
use rand::prelude::SliceRandom;

pub struct RandomizeGenerator<R: 'static + Randomizer + ?Sized> {
    nullable: Nullable,
    children: Vec<(Weight, Box<dyn Generator<R>>)>,
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
            ..
        } = builder;

        if generator_type != GeneratorType::Randomize {
            return Err(CompileError::InvalidType(generator_type));
        }

        match children {
            None => Err(CompileError::EmptyChildren),
            Some(children) => {
                let mut _children: Vec<(Weight, Box<dyn Generator<R>>)> = Vec::new();
                for child_builder in children.into_iter() {
                    let ChildGeneratorBuilder {
                        weight, builder, ..
                    } = child_builder;
                    _children.push((weight.unwrap_or(1), builder.build()?));
                }

                if _children.is_empty() {
                    return Err(CompileError::EmptyChildren);
                }

                if _children.iter().fold(0, |acc, item| acc + item.0) == 0 {
                    return Err(CompileError::AllWeightsZero);
                }

                Ok(Self {
                    nullable,
                    children: _children,
                })
            }
        }
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        value_map: &DataValueMap,
    ) -> Result<DataValue, GenerateError> {
        self.children
            .choose_weighted(rng, |item| item.0)
            .map_err(|err| GenerateError::FailGenerate(err.to_string()))
            .and_then(|item| item.1.generate(rng, value_map))
    }
}
