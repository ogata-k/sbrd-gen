#![deny(missing_debug_implementations)]
//! Module for type of generator

use serde::{Deserialize, Serialize};

/// Type of generator
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum GeneratorType {
    //
    // build string
    //
    /// Type for [`DuplicatePermutationGenerator`]
    ///
    /// [`DuplicatePermutationGenerator`]: ../generator/build_string/struct.DuplicatePermutationGenerator.html
    DuplicatePermutation,
    /// Type for [`FormatGenerator`]
    ///
    /// [`FormatGenerator`]: ../generator/build_string/struct.FormatGenerator.html
    Format,

    //
    // distribution
    //
    /// Type for [`NormalGenerator`]
    ///
    /// [`NormalGenerator`]: ../generator/distribution/struct.NormalGenerator.html
    DistNormal,

    //
    // evaluate
    //
    /// Type for [`EvalGenerator`] as  [`DataValue::Int`]
    ///
    /// [`EvalGenerator`]: ../generator/evaluate/struct.EvalGenerator.html
    /// [`DataValue::Int`]: ../value/enum.DataValue.html#variant.Int
    EvalInt,
    /// Type for [`EvalGenerator`] as  [`DataValue::Real`]
    ///
    /// [`EvalGenerator`]: ../generator/evaluate/struct.EvalGenerator.html
    /// [`DataValue::Real`]: ../value/enum.DataValue.html#variant.Real
    EvalReal,
    /// Type for [`EvalGenerator`] as  [`DataValue::Bool`]
    ///
    /// [`EvalGenerator`]: ../generator/evaluate/struct.EvalGenerator.html
    /// [`DataValue::Bool`]: ../value/enum.DataValue.html#variant.Bool
    EvalBool,
    /// Type for [`EvalGenerator`] as  [`DataValue::String`]
    ///
    /// [`EvalGenerator`]: ../generator/evaluate/struct.EvalGenerator.html
    /// [`DataValue::String`]: ../value/enum.DataValue.html#variant.String
    EvalString,

    //
    // incremental
    //
    /// Type for [`IncrementIdGenerator`]
    ///
    /// [`IncrementIdGenerator`]: ../generator/incremental/struct.IncrementIdGenerator.html
    IncrementId,

    //
    // primitive
    //
    /// Type for [`IntGenerator`]
    ///
    /// [`IntGenerator`]: ../generator/primitive/struct.IntGenerator.html
    Int,
    /// Type for [`RealGenerator`]
    ///
    /// [`RealGenerator`]: ../generator/primitive/struct.RealGenerator.html
    Real,
    /// Type for [`BoolGenerator`]
    ///
    /// [`BoolGenerator`]: ../generator/primitive/struct.BoolGenerator.html
    Bool,
    /// Type for [`DateTimeGenerator`]
    ///
    /// [`DateTimeGenerator`]: ../generator/primitive/struct.DateTimeGenerator.html
    DateTime,
    /// Type for [`DateGenerator`]
    ///
    /// [`DateGenerator`]: ../generator/primitive/struct.DateGenerator.html
    Date,
    /// Type for [`TimeGenerator`]
    ///
    /// [`TimeGenerator`]: ../generator/primitive/struct.TimeGenerator.html
    Time,
    /// Type for [`AlwaysNullGenerator`]
    ///
    /// [`AlwaysNullGenerator`]: ../generator/primitive/struct.AlwaysNullGenerator.html
    AlwaysNull,

    //
    // randomize children
    //
    /// Type for [`CaseWhenGenerator`]
    ///
    /// [`CaseWhenGenerator`]: ../generator/random_children/struct.CaseWhenGenerator.html
    CaseWhen,
    /// Type for [`RandomChildGenerator`]
    ///
    /// [`RandomChildGenerator`]: ../generator/random_children/struct.RandomChildGenerator.html
    RandomChild,

    //
    // randomize values
    //
    /// Type for [`SelectGenerator`] as  [`DataValue::Int`]
    ///
    /// [`SelectGenerator`]: ../generator/random_values/struct.SelectGenerator.html
    /// [`DataValue::Int`]: ../value/enum.DataValue.html#variant.Int
    SelectInt,
    /// Type for [`SelectGenerator`] as  [`DataValue::Real`]
    ///
    /// [`SelectGenerator`]: ../generator/random_values/struct.SelectGenerator.html
    /// [`DataValue::Real`]: ../value/enum.DataValue.html#variant.Real
    SelectReal,
    /// Type for [`SelectGenerator`] as  [`DataValue::String`]
    ///
    /// [`SelectGenerator`]: ../generator/random_values/struct.SelectGenerator.html
    /// [`DataValue::String`]: ../value/enum.DataValue.html#variant.String
    SelectString,
    /// Type for [`GetValueAtGenerator`] as  [`DataValue::Int`]
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/struct.GetValueAtGenerator.html
    /// [`DataValue::Int`]: ../value/enum.DataValue.html#variant.Int
    GetIntValueAt,
    /// Type for [`GetValueAtGenerator`] as  [`DataValue::Real`]
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/struct.GetValueAtGenerator.html
    /// [`DataValue::Real`]: ../value/enum.DataValue.html#variant.Real
    GetRealValueAt,
    /// Type for [`GetValueAtGenerator`] as  [`DataValue::String`]
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/struct.GetValueAtGenerator.html
    /// [`DataValue::String`]: ../value/enum.DataValue.html#variant.String
    GetStringValueAt,
    /// Type for [`GetValueIndexGenerator`]
    ///
    /// [`GetValueIndexGenerator`]: ../generator/random_values/struct.GetValueIndexGenerator.html
    GetValueIndex,
}

impl std::fmt::Display for GeneratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_yaml::to_string(&self)
            .unwrap_or_else(|e| panic!("Fail serialize {:?} with error {}.", &self, e));
        write!(f, "{}", s)
    }
}
