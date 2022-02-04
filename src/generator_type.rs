use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum GeneratorType {
    // primitive
    Int,
    Real,
    Bool,
    AlwaysNull,
    // evaluate
    EvalInt,
    EvalReal,
    // format
    Format,
    // random select
    SelectInt,
    SelectReal,
    SelectString,
    // distribution
    DistIntUniform,
    DistRealUniform,
    DistRealNormal,
    // case-when condition. default value is null.
    When,
    // date, time and datetime
    DateTime,
    Date,
    Time,
    // increment value
    IncrementId,
}
