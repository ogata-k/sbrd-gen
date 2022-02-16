use crate::value::{DataValueMap, SbrdBool, SbrdInt, SbrdReal, SbrdString};
use evalexpr::{eval_boolean, eval_float, eval_int, EvalexprError};

#[derive(Debug, PartialEq, Clone)]
pub struct Evaluator<'a> {
    script: &'a str,
    context: &'a DataValueMap<&'a str>,
}

pub type EvalError = EvalexprError;
pub type EvalResult<T> = Result<T, EvalError>;
impl<'a> Evaluator<'a> {
    /// scriptはcontextの各エントリー`(key, value)`をもとに"{key}"を`value`で置き換える形で処理される
    pub fn new(script: &'a str, context: &'a DataValueMap<&str>) -> Self {
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
