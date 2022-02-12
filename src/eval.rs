use crate::{DataValue, DataValueMap, SbrdBool, SbrdInt, SbrdReal, SbrdString};
use evalexpr::{eval, eval_boolean, eval_float, eval_int, EvalexprError, Value};

#[derive(Debug, PartialEq, Clone)]
pub struct Evaluator<'a> {
    script: &'a str,
    context: &'a DataValueMap,
}

pub type EvalError = EvalexprError;
pub type EvalResult<T> = Result<T, EvalError>;
impl<'a> Evaluator<'a> {
    /// scriptはcontextの各エントリー`(key, value)`をもとに"{key}"を`value`で置き換える形で処理される
    pub fn new(script: &'a str, context: &'a DataValueMap) -> Self {
        Self { script, context }
    }

    pub fn format_script(&self) -> SbrdString {
        let mut replaced_script = self.script.to_string();
        for (key, value) in self.context.iter() {
            // formatは{key}をvalueで置換して表示する
            let format = format!("{{{}}}", key);
            let eval_value = value.to_format_value();
            replaced_script = replaced_script.replace(&format, &eval_value);
        }

        replaced_script
    }

    pub fn eval_data_value(&self) -> EvalResult<DataValue> {
        match eval(&self.format_script()) {
            Ok(eval_value) => match eval_value {
                Value::String(v) => Ok(DataValue::String(v)),
                Value::Float(v) => Ok(DataValue::Real(v as SbrdReal)),
                Value::Int(v) => Ok(DataValue::Int(v as SbrdInt)),
                Value::Boolean(v) => Ok(DataValue::Bool(v)),
                Value::Tuple(_) => Err(EvalexprError::expected_number_or_string(eval_value)),
                Value::Empty => Ok(DataValue::Null),
            },
            Err(e) => Err(e),
        }
    }

    pub fn eval_int(&self) -> EvalResult<SbrdInt> {
        eval_int(&self.format_script()).map(|v| v as SbrdInt)
    }

    pub fn eval_real(&self) -> EvalResult<SbrdReal> {
        eval_float(&self.format_script()).map(|v| v as SbrdReal)
    }

    pub fn eval_bool(&self) -> EvalResult<SbrdBool> {
        eval_boolean(&self.format_script()).map(|v| v as SbrdBool)
    }
}
