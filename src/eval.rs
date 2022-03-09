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
    context: &'a DataValueMap<&'a str>,
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
        Self { script, context }
    }

    /// Create context for this evaluator
    fn create_context() -> EvalResult<EvalContext> {
        let mut context = EvalContext::new();
        fn get_at_function(ordinal_number: usize) -> Function {
            Function::new(move |argument| {
                let values = argument.as_tuple()?;
                match values.get(ordinal_number - 1) {
                    None => Err(EvalexprError::CustomMessage(format!(
                        "Not found value in {} at tuple index {}",
                        argument,
                        ordinal_number - 1
                    ))),
                    Some(value) => Ok(value.clone()),
                }
            })
        }

        context.set_function("first".to_string(), get_at_function(1))?;
        context.set_function("second".to_string(), get_at_function(2))?;
        context.set_function("third".to_string(), get_at_function(3))?;
        context.set_function("fourth".to_string(), get_at_function(4))?;
        context.set_function("fifth".to_string(), get_at_function(5))?;
        context.set_function("sixth".to_string(), get_at_function(6))?;
        context.set_function("seventh".to_string(), get_at_function(7))?;
        context.set_function("eighth".to_string(), get_at_function(8))?;
        context.set_function("ninth".to_string(), get_at_function(9))?;
        context.set_function("tenth".to_string(), get_at_function(10))?;

        Ok(context)
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
        eval_int_with_context_mut(&self.format_script()?, &mut Self::create_context()?)
            .map(|v| v as SbrdInt)
    }

    /// Evaluate the script applied the context, as [`SbrdReal`]
    ///
    /// [`SbrdReal`]: ../value/type.SbrdReal.html
    pub fn eval_real(&self) -> EvalResult<SbrdReal> {
        eval_float_with_context_mut(&self.format_script()?, &mut Self::create_context()?)
            .map(|v| v as SbrdReal)
    }

    /// Evaluate the script applied the context, as [`SbrdBool`]
    ///
    /// [`SbrdBool`]: ../value/type.SbrdBool.html
    pub fn eval_bool(&self) -> EvalResult<SbrdBool> {
        eval_boolean_with_context_mut(&self.format_script()?, &mut Self::create_context()?)
            .map(|v| v as SbrdBool)
    }

    /// Evaluate the script applied the context, as [`SbrdString`]
    ///
    /// [`SbrdString`]: ../value/type.SbrdString.html
    pub fn eval_string(&self) -> EvalResult<SbrdString> {
        eval_string_with_context_mut(&self.format_script()?, &mut Self::create_context()?)
    }
}
