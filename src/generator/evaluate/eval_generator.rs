use crate::builder::{GeneratorBuilder, Nullable};
use crate::error::{BuildError, GenerateError};
use crate::eval::{EvalResult, Evaluator};
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap, SbrdBool, SbrdInt, SbrdReal};
use crate::GeneratorType;
use std::marker::PhantomData;

/// The generator with generate with evaluate generated value
#[derive(Debug, PartialEq, Clone)]
pub struct EvalGenerator<T> {
    nullable: Nullable,
    script: String,
    _calculated_type: PhantomData<T>,
}

impl<R: Randomizer + ?Sized, F: ForEvalGeneratorType> Generator<R> for EvalGenerator<F> {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            script,
            ..
        } = builder;

        if generator_type != F::get_generator_type() {
            return Err(BuildError::InvalidType(generator_type));
        }

        match script {
            None => Err(BuildError::NotExistValueOf("script".to_string())),
            Some(_script) => Ok(Self {
                nullable,
                script: _script,
                _calculated_type: PhantomData,
            }),
        }
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        _rng: &mut R,
        value_map: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        F::eval_script(&self.script, value_map).map_err(|e| {
            GenerateError::FailEval(
                e,
                self.script.clone(),
                value_map
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.clone()))
                    .collect::<DataValueMap<String>>(),
            )
        })
    }
}

/// Helper traits for generators that generate evaluable values
pub trait ForEvalGeneratorType {
    /// The type of the generator
    fn get_generator_type() -> GeneratorType;

    /// evaluate the script with the context
    fn eval_script<'a>(script: &'a str, context: &'a DataValueMap<&str>) -> EvalResult<DataValue>;
}

impl ForEvalGeneratorType for SbrdInt {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalInt
    }

    fn eval_script<'a>(script: &'a str, context: &'a DataValueMap<&str>) -> EvalResult<DataValue> {
        let evaluator = Evaluator::new(script, context);
        evaluator.eval_int().map(|v| v.into())
    }
}
impl ForEvalGeneratorType for SbrdReal {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalReal
    }

    fn eval_script<'a>(script: &'a str, context: &'a DataValueMap<&str>) -> EvalResult<DataValue> {
        let evaluator = Evaluator::new(script, context);
        evaluator.eval_real().map(|v| v.into())
    }
}
impl ForEvalGeneratorType for SbrdBool {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalBool
    }

    fn eval_script<'a>(script: &'a str, context: &'a DataValueMap<&str>) -> EvalResult<DataValue> {
        let evaluator = Evaluator::new(script, context);
        evaluator.eval_bool().map(|v| v.into())
    }
}
