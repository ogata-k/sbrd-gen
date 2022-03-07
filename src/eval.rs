#![deny(missing_debug_implementations)]
//! Module for evaluator for `script` and `format`

use crate::value::{DataValueMap, SbrdBool, SbrdInt, SbrdReal, SbrdString};
use evalexpr::{eval_boolean, eval_float, eval_int, EvalexprError};

/// Evaluator for `script` and `format`
/// Script and format is processed by replacing "{key}" with value based on each entry `(key, value)` of context.
/// If you'll know syntax and available functions more, you can see [`this document`]
///
/// [`this document`]: https://docs.rs/evalexpr/7.0.1/evalexpr/index.html#features
#[derive(Debug, PartialEq, Clone)]
pub struct Evaluator<'a> {
    script: &'a str,
    context: &'a DataValueMap<&'a str>,
}

/// Error while evaluate
pub type EvalError = EvalexprError;
/// Alias of [`Result`] for [`Evaluator`]
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`Evaluator`]: ./struct.Evaluator.html
pub type EvalResult<T> = Result<T, EvalError>;

impl<'a> Evaluator<'a> {
    /// Create from script and context
    pub fn new(script: &'a str, context: &'a DataValueMap<&str>) -> Self {
        Self { script, context }
    }

    /// Apply context to the script
    pub fn format_script(&self) -> EvalResult<SbrdString> {
        // @todo スクリプトのフォーマットの正当性を判定できるようにしたい
        let mut replaced_script = self.script.to_string();
        for (key, value) in self.context.iter() {
            // formatは{key}をvalueで置換して表示する
            let format = format!("{{{}}}", key);
            let eval_value = value.to_format_value();
            replaced_script = replaced_script.replace(&format, &eval_value);
        }

        Ok(replaced_script)
    }

    /// Evaluate the script applied the context, as [`SbrdInt`]
    ///
    /// [`SbrdInt`]: ../value/type.SbrdInt.html
    pub fn eval_int(&self) -> EvalResult<SbrdInt> {
        eval_int(&self.format_script()?).map(|v| v as SbrdInt)
    }

    /// Evaluate the script applied the context, as [`SbrdReal`]
    ///
    /// [`SbrdReal`]: ../value/type.SbrdReal.html
    pub fn eval_real(&self) -> EvalResult<SbrdReal> {
        eval_float(&self.format_script()?).map(|v| v as SbrdReal)
    }

    /// Evaluate the script applied the context, as [`SbrdBool`]
    ///
    /// [`SbrdBool`]: ../value/type.SbrdBool.html
    pub fn eval_bool(&self) -> EvalResult<SbrdBool> {
        eval_boolean(&self.format_script()?).map(|v| v as SbrdBool)
    }
}
