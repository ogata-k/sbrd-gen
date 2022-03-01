//! Module for step

use serde::{Deserialize, Serialize};

/// Value step option
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct ValueStep<T> {
    /// Initial value
    initial: T,
    /// Step value
    #[serde(skip_serializing_if = "Option::is_none")]
    step: Option<T>,
}

impl<T> ValueStep<T> {
    /// Create ValueStep
    pub fn new(initial: T, step: Option<T>) -> Self {
        Self { initial, step }
    }

    /// Get initial value
    pub fn get_initial(&self) -> &T {
        &self.initial
    }

    /// Get step value
    pub fn get_step(&self) -> &Option<T> {
        &self.step
    }

    /// Convert into other with into-method.
    pub fn convert_into<U>(self) -> ValueStep<U>
    where
        T: Into<U>,
    {
        self.convert_with(|v| v.into())
    }

    /// Convert into other with custom-method
    pub fn convert_with<F, U>(self, mut convert: F) -> ValueStep<U>
    where
        F: FnMut(T) -> U,
    {
        let Self { initial, step } = self;

        ValueStep {
            initial: convert(initial),
            step: step.map(|e| {
                #[allow(clippy::redundant_closure)]
                convert(e)
            }),
        }
    }

    /// Try convert into other with custom-method
    pub fn try_convert_with<F, U, E>(self, mut convert: F) -> Result<ValueStep<U>, E>
    where
        F: FnMut(T) -> Result<U, E>,
    {
        let Self { initial, step } = self;

        let _step = match step {
            None => None,
            Some(step) => Some(convert(step)?),
        };

        Ok(ValueStep {
            initial: convert(initial)?,
            step: _step,
        })
    }
}

impl<T: std::fmt::Display> std::fmt::Display for ValueStep<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { initial, step } = &self;

        match step {
            Some(_step) => write!(f, "{}..(/{})", initial, _step),
            None => write!(f, "{}..", initial),
        }
    }
}
