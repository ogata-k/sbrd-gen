#![deny(missing_debug_implementations)]
//! Module for evaluator for `script` and `format`

use crate::value::{DataValueMap, SbrdBool, SbrdInt, SbrdReal, SbrdString};
use evalexpr::{
    eval_boolean_with_context_mut, eval_float_with_context_mut, eval_int_with_context_mut,
    eval_string_with_context_mut, ContextWithMutableFunctions, EvalexprError, Function,
    HashMapContext,
};

/// Evaluator for `script` and `format`
/// Script and format is processed by replacing "{key}" with value based on each entry `(key, value)` of context.
/// If you'll know syntax and available functions more, you can see [`this document`]
///
/// [`this document`]: https://docs.rs/evalexpr/7.0.1/evalexpr/index.html#features
#[derive(Debug, PartialEq, Clone)]
pub struct Evaluator<'a> {
    script: &'a str,
    script_context: &'a DataValueMap<&'a str>,
}

/// Context for evaluator
pub type EvalContext = HashMapContext;
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
        Self {
            script,
            script_context: context,
        }
    }

    /// Create context when use evaluate
    fn create_eval_context() -> EvalResult<EvalContext> {
        let mut context = EvalContext::new();

        // @todo replace evalexpr-crate's function for get value at the index
        context.set_function("get".to_string(),  Function::new(move |argument| {
            let arg_tuple = argument.as_fixed_len_tuple(2)?;
            let (values, index) = (arg_tuple[0].as_tuple()?, arg_tuple[1].as_int()?);

            if index < 0 {
                return Err(EvalexprError::CustomMessage("Invalid index in a script.".to_string()));
            }
            match values.get(index as usize) {
                None => Err(EvalexprError::CustomMessage(format!(
                    "Not found value in {} at tuple index {}",
                    argument,
                    index
                ))),
                Some(value) => Ok(value.clone()),
            }
        }))?;

        Ok(context)
    }

    /// Apply value-context to the script
    pub fn format_script(&self) -> EvalResult<String> {
        // @todo スクリプトのフォーマットの正当性を判定できるようにしたい
        let mut replaced_script = self.script.to_string();
        for (key, value) in self.script_context.iter() {
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
        eval_int_with_context_mut(&self.format_script()?, &mut Self::create_eval_context()?)
            .map(|v| v as SbrdInt)
    }

    /// Evaluate the script applied the context, as [`SbrdReal`]
    ///
    /// [`SbrdReal`]: ../value/type.SbrdReal.html
    pub fn eval_real(&self) -> EvalResult<SbrdReal> {
        eval_float_with_context_mut(&self.format_script()?, &mut Self::create_eval_context()?)
            .map(|v| v as SbrdReal)
    }

    /// Evaluate the script applied the context, as [`SbrdBool`]
    ///
    /// [`SbrdBool`]: ../value/type.SbrdBool.html
    pub fn eval_bool(&self) -> EvalResult<SbrdBool> {
        eval_boolean_with_context_mut(&self.format_script()?, &mut Self::create_eval_context()?)
            .map(|v| v as SbrdBool)
    }

    /// Evaluate the script applied the context, as [`SbrdString`]
    ///
    /// [`SbrdString`]: ../value/type.SbrdString.html
    pub fn eval_string(&self) -> EvalResult<SbrdString> {
        eval_string_with_context_mut(&self.format_script()?, &mut Self::create_eval_context()?)
    }
}
