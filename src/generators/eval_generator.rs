use std::marker::PhantomData;
use std::str::FromStr;

use eval::eval;
use rand::Rng;

use crate::generators::error::{CompileError, GenerateError};
use crate::generators::Generator;
use crate::{
    replace_values, DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable, SbrdBool,
    SbrdInt, SbrdReal,
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
        let mut _script = replace_values(&self.script, value_map);
        let eval_result =
            eval(&_script).map_err(|e| GenerateError::FailEval(e, _script, self.script.clone()))?;

        F::eval_value_to_data_value(eval_result)
    }
}

pub trait ForEvalGeneratorType: FromStr {
    fn get_generator_type() -> GeneratorType;
    fn eval_value_to_data_value(value: eval::Value) -> Result<DataValue, GenerateError>;
}

impl ForEvalGeneratorType for SbrdInt {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalInt
    }

    fn eval_value_to_data_value(value: eval::Value) -> Result<DataValue, GenerateError> {
        match value.as_i64() {
            None => Err(GenerateError::FailGenerate(format!(
                "Invalid Value: {}",
                value
            ))),
            Some(v) => match SbrdInt::try_from(v) {
                Ok(v) => Ok(DataValue::Int(v)),
                Err(e) => Err(GenerateError::FailGenerate(e.to_string())),
            },
        }
    }
}
impl ForEvalGeneratorType for SbrdReal {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalReal
    }

    fn eval_value_to_data_value(value: eval::Value) -> Result<DataValue, GenerateError> {
        match value.as_f64() {
            None => Err(GenerateError::FailGenerate(format!(
                "Invalid Value: {}",
                value
            ))),
            // @todo エラーなしに変換する方法がわからないのでエラーになってもしょうがないと割り切ったが、方法があるなら置き換える
            Some(v) => Ok(DataValue::Real(v as SbrdReal)),
        }
    }
}
impl ForEvalGeneratorType for SbrdBool {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::EvalBool
    }

    fn eval_value_to_data_value(value: eval::Value) -> Result<DataValue, GenerateError> {
        match value.as_bool() {
            None => Err(GenerateError::FailGenerate(format!(
                "Invalid Value: {}",
                value
            ))),
            Some(v) => Ok(DataValue::Bool(v)),
        }
    }
}
