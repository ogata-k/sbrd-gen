use std::marker::PhantomData;
use std::str::FromStr;

use rand::Rng;

use crate::eval::Evaluator;
use crate::generators::error::{CompileError, GenerateError};
use crate::generators::Generator;
use crate::{
    DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable, SbrdBool, SbrdInt, SbrdReal,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct EvalGenerator<T: FromStr> {
    nullable: Nullable,
    /// Supported operators: ! != "" '' () [] . , > < >= <= == + - * / % && || n..m.
    ///
    /// Built-in functions: min() max() len() is_empty() array().
    script: String,
    _calculated_type: PhantomData<T>,
}

impl<R: Rng + ?Sized, F: ForEvalGeneratorType> Generator<R> for EvalGenerator<F> {
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
        let evaluator = Evaluator::new(&self.script, value_map);
        evaluator
            .eval_data_value()
            .map_err(|e| GenerateError::FailEval(e, self.script.clone(), value_map.clone()))
    }
}

pub trait ForEvalGeneratorType: FromStr {
    fn get_generator_type() -> GeneratorType;
}

impl ForEvalGeneratorType for SbrdInt {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalInt
    }
}
impl ForEvalGeneratorType for SbrdReal {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalReal
    }
}
impl ForEvalGeneratorType for SbrdBool {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalBool
    }
}
