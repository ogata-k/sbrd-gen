use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum GeneratorType {
    Int,
    Real,
    Bool,
    AlwaysNull,
    UnsignedInt,
    UnsignedReal,
    EvalInt,
    EvalReal,
    Format,
    SelectInt,
    SelectReal,
    SelectString,
    DistIntUniform,
    DistRealUniform,
    DistRealNormal,
    When,
    IncrementId, // 下限のみサポート
    DateTime,
    Date,
    Time,
}
