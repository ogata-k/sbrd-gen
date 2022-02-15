use crate::builder::{GeneratorBuilder, Nullable};
use crate::eval::{EvalResult, Evaluator};
use crate::generator::error::{CompileError, GenerateError};
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap, SbrdBool, SbrdInt, SbrdReal};
use crate::GeneratorType;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct EvalGenerator<T> {
    nullable: Nullable,
    script: String,
    _calculated_type: PhantomData<T>,
}

impl<R: Randomizer + ?Sized, F: ForEvalGeneratorType> Generator<R> for EvalGenerator<F> {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
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
            return Err(CompileError::InvalidType(generator_type));
        }

        match script {
            None => Err(CompileError::NotExistValueOf("script".to_string())),
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
        value_map: &DataValueMap,
    ) -> Result<DataValue, GenerateError> {
        F::evaluate(&self.script, value_map)
            .map_err(|e| GenerateError::FailEval(e, self.script.clone(), value_map.clone()))
    }
}

pub trait ForEvalGeneratorType {
    fn get_generator_type() -> GeneratorType;

    fn evaluate<'a>(script: &'a str, context: &'a DataValueMap) -> EvalResult<DataValue>;
}

impl ForEvalGeneratorType for SbrdInt {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalInt
    }

    fn evaluate<'a>(script: &'a str, context: &'a DataValueMap) -> EvalResult<DataValue> {
        let evaluator = Evaluator::new(script, context);
        evaluator.eval_int().map(|v| v.into())
    }
}
impl ForEvalGeneratorType for SbrdReal {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalReal
    }

    fn evaluate<'a>(script: &'a str, context: &'a DataValueMap) -> EvalResult<DataValue> {
        let evaluator = Evaluator::new(script, context);
        evaluator.eval_real().map(|v| v.into())
    }
}
impl ForEvalGeneratorType for SbrdBool {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalBool
    }

    fn evaluate<'a>(script: &'a str, context: &'a DataValueMap) -> EvalResult<DataValue> {
        let evaluator = Evaluator::new(script, context);
        evaluator.eval_bool().map(|v| v.into())
    }
}
