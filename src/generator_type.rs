use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum GeneratorType {
    // primitive
    Int,
    Real,
    Bool,
    DateTime,
    Date,
    Time,
    // special primitive
    AlwaysNull,
    IncrementId,
    // evaluate
    EvalInt,
    EvalReal,
    EvalBool,
    // join
    Format,
    // join (use bound parameter as count)
    DuplicatePermutation,
    // random select
    CaseWhen,
    SelectInt,
    SelectReal,
    SelectString,
    // distribution
    DistIntUniform,
    DistRealUniform,
    DistRealNormal,
}

impl std::fmt::Display for GeneratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_yaml::to_string(&self)
            .unwrap_or_else(|e| panic!("Fail serialize {:?} with error {}.", &self, e));
        write!(f, "{}", s)
    }
}
