#![deny(missing_debug_implementations)]

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum GeneratorType {
    //
    // build string
    //
    DuplicatePermutation,
    Format,

    //
    // distribution
    //
    DistNormal,

    //
    // evaluate
    //
    EvalInt,
    EvalReal,
    EvalBool,

    //
    // primitive
    //
    Int,
    Real,
    Bool,
    DateTime,
    Date,
    Time,
    AlwaysNull,
    IncrementId,

    //
    // randomize children
    //
    CaseWhen,

    //
    // randomize values
    //
    SelectInt,
    SelectReal,
    SelectString,

    //
    // random values and children
    //
    Randomize,
}

impl std::fmt::Display for GeneratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_yaml::to_string(&self)
            .unwrap_or_else(|e| panic!("Fail serialize {:?} with error {}.", &self, e));
        write!(f, "{}", s)
    }
}
