#![deny(missing_debug_implementations)]
//! Module for evaluator for `script` and `format`

use crate::value::{DataValueMap, SbrdBool, SbrdInt, SbrdReal, SbrdString};
use evalexpr::{
    eval_boolean_with_context_mut, eval_float_with_context_mut, eval_int_with_context_mut,
    eval_string_with_context_mut, ContextWithMutableFunctions, EvalexprError, Function,
    HashMapContext,
};
use human_string_filler::StrExt;
use std::fmt::Write;

/// Evaluator for `script` and `format`.
/// Script and format is processed by replacing a replace-key-syntax for the key with value based on each entry `(key, value)` of context.
/// Replace-key-syntax is "{key}" and "{key:\<format-option\>}". It specified by Rust format syntax with the key as name position. But not support index position, variable, padding with character and [`Pointer`] format (`{:p}`).
/// [`Debug`] format is not supported in release build.
/// If you want to know, you will see [`Rust-format syntax`] and [`DataValue::format`].
///
/// All values, variables and functions are available as described in the [`evalexpr`] except the regex functions.
/// If you'll know syntax and available them more, you can see [`this document`].
///
/// # Examples
/// ```
/// fn main(){
///     use sbrd_gen::eval::Evaluator;
///     use sbrd_gen::value::{DataValue, DataValueMap};
///
///     let mut value_context = DataValueMap::new();
///     value_context.insert("Key-Int", DataValue::Int(12));
///     value_context.insert("キー Real", DataValue::Real(12.345));
///     value_context.insert("Key:String", DataValue::String("aiueoあいうえお".to_string()));
///     value_context.insert("Key Bool:", DataValue::Bool(true));
///     value_context.insert("key Null ", DataValue::Null);
///     let evaluator = Evaluator::new(&value_context);
///
///     assert_eq!(Ok("no key".to_string()), evaluator.format_script("no key"));
///     assert_eq!(Ok("12".to_string()), evaluator.format_script("{Key-Int}"));
///     assert_eq!(Ok("Rate= +12.35".to_string()), evaluator.format_script("Rate={キー Real:+7.2}"));
///     assert_eq!(Ok("Rate=+012.35".to_string()), evaluator.format_script("Rate={キー Real:+07.2}"));
///     assert_eq!(Ok(" aiueoあいうえお ".to_string()), evaluator.format_script("{Key:String:^12}"));
///     assert_eq!(Ok("true    ".to_string()), evaluator.format_script("{Key Bool::<8}"));
///     assert_eq!(Ok("".to_string()), evaluator.format_script("{key Null }"));
/// }
/// ```
///
/// [`Rust-format syntax`]: https://doc.rust-lang.org/std/fmt/index.html#syntax
/// [`Pointer`]: https://doc.rust-lang.org/std/fmt/trait.Pointer.html
/// [`Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
/// [`DataValue::format`]: ../value/enum.DataValue.html#method.format
/// [`evalexpr`]: https://crates.io/crates/evalexpr/7.0.1
/// [`this document`]: https://docs.rs/evalexpr/7.0.1/evalexpr/index.html#features
#[derive(Debug, PartialEq, Clone)]
pub struct Evaluator<'a> {
    value_context: &'a DataValueMap<&'a str>,
}

/// Context for evaluator
pub type EvalContext = HashMapContext;
/// Error while evaluate
#[derive(Debug, PartialEq)]
pub enum EvalError {
    /// Fail evaluate
    FailEval(EvalexprError),
    /// Fail apply value context
    FailApplyValueContext(String),
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::FailEval(e) => write!(f, "Fail eval with error: {}", e),
            EvalError::FailApplyValueContext(e) => {
                write!(f, "Fail apply value context with error: {}", e)
            }
        }
    }
}

impl std::error::Error for EvalError {}

/// Alias of [`Result`] for [`Evaluator`]
///
/// [`Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`Evaluator`]: ./struct.Evaluator.html
pub type EvalResult<T> = Result<T, EvalError>;

impl<'a> Evaluator<'a> {
    /// Create from script and a value context
    pub fn new(value_context: &'a DataValueMap<&str>) -> Self {
        Self { value_context }
    }

    /// Create context when use evaluate
    fn create_eval_context() -> EvalResult<EvalContext> {
        let mut context = EvalContext::new();

        // @todo replace evalexpr-crate's function for get value at the index
        context
            .set_function(
                "get".to_string(),
                Function::new(move |argument| {
                    let arg_tuple = argument.as_fixed_len_tuple(2)?;
                    let (values, index) = (arg_tuple[0].as_tuple()?, arg_tuple[1].as_int()?);

                    if index < 0 {
                        return Err(EvalexprError::CustomMessage(
                            "Invalid index in a script.".to_string(),
                        ));
                    }
                    match values.get(index as usize) {
                        None => Err(EvalexprError::CustomMessage(format!(
                            "Not found value in {} at tuple index {}",
                            argument, index
                        ))),
                        Some(value) => Ok(value.clone()),
                    }
                }),
            )
            .map_err(EvalError::FailEval)?;

        Ok(context)
    }

    /// Apply value-context to the script
    fn apply_value_context(&self, script: &str) -> EvalResult<String> {
        let mut result = String::new();
        script
            .fill_into::<_, _, String>(&mut result, |output: &mut String, key: &str| {
                match self.value_context.get(key) {
                    Some(v) => {
                        let formatted = v
                            .format("{}")
                            .ok_or_else(|| format!("Fail apply key \"{}\".", key))?;

                        output.write_str(&formatted).map_err(|e| e.to_string())?;
                    }
                    None => {
                        let split_index: usize = key.rfind(':').ok_or_else(|| {
                            format!("Not found key \"{}\" at the value context.", key)
                        })?;
                        let _key = &key[0..split_index];
                        match self.value_context.get(_key) {
                            Some(v) => {
                                let formatted = v
                                    .format(&format!("{{{}}}", &key[split_index..key.len()]))
                                    .ok_or_else(|| format!("Fail apply key \"{}\".", _key))?;

                                output.write_str(&formatted).map_err(|e| e.to_string())?;
                            }
                            None => {
                                return Err(format!(
                                    "Not found key \"{}\" at the value context.",
                                    _key
                                ));
                            }
                        }
                    }
                }
                Ok(())
            })
            .map(|_| result)
            .map_err(|e| EvalError::FailApplyValueContext(e.to_string()))
    }

    /// Get format applied value-context to the script.
    ///
    /// If you want to know syntax, you will see [`Evaluator`]'s document.
    ///
    /// [`Evaluator`]: ./struct.Evaluator.html
    pub fn format_script(&self, script: &str) -> EvalResult<String> {
        self.apply_value_context(script)
    }

    /// Evaluate the script applied the context, as [`SbrdInt`]
    ///
    /// [`SbrdInt`]: ../value/type.SbrdInt.html
    pub fn eval_int(&self, script: &str) -> EvalResult<SbrdInt> {
        eval_int_with_context_mut(
            &self.format_script(script)?,
            &mut Self::create_eval_context()?,
        )
        .map(|v| v as SbrdInt)
        .map_err(EvalError::FailEval)
    }

    /// Evaluate the script applied the context, as [`SbrdReal`]
    ///
    /// [`SbrdReal`]: ../value/type.SbrdReal.html
    pub fn eval_real(&self, script: &str) -> EvalResult<SbrdReal> {
        eval_float_with_context_mut(
            &self.format_script(script)?,
            &mut Self::create_eval_context()?,
        )
        .map(|v| v as SbrdReal)
        .map_err(EvalError::FailEval)
    }

    /// Evaluate the script applied the context, as [`SbrdBool`]
    ///
    /// [`SbrdBool`]: ../value/type.SbrdBool.html
    pub fn eval_bool(&self, script: &str) -> EvalResult<SbrdBool> {
        eval_boolean_with_context_mut(
            &self.format_script(script)?,
            &mut Self::create_eval_context()?,
        )
        .map(|v| v as SbrdBool)
        .map_err(EvalError::FailEval)
    }

    /// Evaluate the script applied the context, as [`SbrdString`]
    ///
    /// [`SbrdString`]: ../value/type.SbrdString.html
    pub fn eval_string(&self, script: &str) -> EvalResult<SbrdString> {
        eval_string_with_context_mut(
            &self.format_script(script)?,
            &mut Self::create_eval_context()?,
        )
        .map_err(EvalError::FailEval)
    }
}
