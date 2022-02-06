use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
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
    // join
    Format,
    // join (use bound parameter as count)
    DuplicatePermutation,
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

impl std::fmt::Display for GeneratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_yaml::to_string(&self)
            .unwrap_or_else(|e| panic!("Fail serialize {:?} with error {}.", &self, e));
        write!(f, "{}", s)
    }
}
