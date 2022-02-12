use rand::Rng;

use crate::eval::Evaluator;
use crate::generators::error::{CompileError, GenerateError};
use crate::generators::Generator;
use crate::{DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable};

pub struct CaseWhenGenerator<R: 'static + Rng + ?Sized> {
    nullable: Nullable,
    children: Vec<(Option<String>, Box<dyn Generator<R>>)>,
}

impl<R: Rng + ?Sized> Generator<R> for CaseWhenGenerator<R> {
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

        if generator_type != GeneratorType::CaseWhen {
            return Err(CompileError::InvalidType(generator_type));
        }

        match children {
            None => Err(CompileError::EmptyChildren),
            Some(children) => {
                let mut _children: Vec<(Option<String>, Box<dyn Generator<R>>)> = Vec::new();
                let mut has_default_case = false;
                for with_condition_builder in children.into_iter() {
                    let (condition, builder) = with_condition_builder.split();
                    has_default_case = has_default_case || condition.is_none();
                    _children.push((condition, builder.build()?));
                }

                if !has_default_case {
                    return Err(CompileError::NotExistDefaultCase);
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
        for (condition, generator) in self.children.iter() {
            return match condition {
                None => generator.generate(rng, value_map),
                Some(_condition) => {
                    let evaluator = Evaluator::new(_condition, value_map);
                    let is_satisfy = evaluator.eval_bool().map_err(|e| {
                        GenerateError::FailEval(e, _condition.clone(), value_map.clone())
                    })?;
                    if !is_satisfy {
                        continue;
                    }

                    generator.generate(rng, value_map)
                }
            };
        }

        Err(GenerateError::FailGenerate(
            "No match case condition".to_string(),
        ))
    }
}
