use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum GeneratorType {
    Int,
    Real,
    Bool,
    UnsignedInt,
    UnsignedReal,
    EvalInt,
    EvalReal,
    Format,
    Select,
    DistIntUniform,
    DistRealUniform,
    DistRealNormal,
    When,
    IncrementId,
}
