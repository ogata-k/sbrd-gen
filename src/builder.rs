use std::collections::btree_map::BTreeMap;
use std::path::PathBuf;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

use crate::bound::ValueBound;
use crate::generator_type::GeneratorType;
use crate::Nullable;
use crate::value::DataValue;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct GeneratorBuilder {
    #[serde(rename = "type")]
    generator_type: GeneratorType,
    #[serde(
        skip_serializing_if = "Nullable::is_required",
        default = "Nullable::new_required"
    )]
    nullable: Nullable,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<ValueBound<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<GeneratorBuilder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chars: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    separator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parameters: Option<BTreeMap<String, DataValue>>,
}

impl GeneratorBuilder {
    //
    // constructor functions following:
    //

    fn new(generator_type: GeneratorType) -> Self {
        Self {
            key: None,
            condition: None,
            generator_type,
            nullable: Nullable::new_required(),
            range: None,
            file: None,
            separator: None,
            values: None,
            format: None,
            chars: None,
            parameters: None,
            children: None,
        }
    }

    pub fn new_int(range: Option<ValueBound<u64>>) -> Self {
        let mut this = Self::new(GeneratorType::Int);
        if let Some(range) = range {
            this = this.range(range.convert_with(|v| format!("{}", v)));
        }

        this
    }

    pub fn new_real(range: Option<ValueBound<f64>>) -> Self {
        let mut this = Self::new(GeneratorType::Real);
        if let Some(range) = range {
            this = this.range(range.convert_with(|v| format!("{}", v)));
        }

        this
    }

    pub fn new_bool() -> Self {
        Self::new(GeneratorType::Bool)
    }

    pub fn new_always_null() -> Self {
        Self::new(GeneratorType::AlwaysNull)
    }

    pub fn new_eval_int<S>(format: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::EvalInt).format(format.into())
    }

    pub fn new_eval_real<S>(format: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::EvalReal).format(format.into())
    }

    pub fn new_format<S>(format: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::Format).format(format.into())
    }

    fn new_duplicate_permutation<S>(range: Option<ValueBound<u64>>, separator: S) -> Self
    where
        S: Into<String>,
    {
        let mut this = Self::new(GeneratorType::DuplicatePermutation).separator(separator.into());
        if let Some(range) = range {
            this = this.range(range.convert_with(|v| format!("{}", v)));
        }

        this
    }

    pub fn new_duplicate_permutation_with_chars<S1, S2>(
        range: Option<ValueBound<u64>>,
        separator: S1,
        chars: S2,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self::new_duplicate_permutation(range, separator).chars(chars.into())
    }

    pub fn new_duplicate_permutation_with_children<S, V>(
        range: Option<ValueBound<u64>>,
        separator: S,
        builders: V,
    ) -> Self
    where
        S: Into<String>,
        V: Into<Vec<GeneratorBuilder>>,
    {
        Self::new_duplicate_permutation(range, separator).children(builders)
    }

    fn new_select_int() -> Self {
        Self::new(GeneratorType::SelectInt)
    }

    pub fn new_select_int_with_values<V>(values: V) -> Self
    where
        V: Into<Vec<u64>>,
    {
        Self::new_select_int().values(
            values
                .into()
                .into_iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>(),
        )
    }

    pub fn new_select_int_with_file<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self::new_select_int().file(path)
    }

    fn new_select_real() -> Self {
        Self::new(GeneratorType::SelectReal)
    }

    pub fn new_select_real_with_values<V>(values: V) -> Self
    where
        V: Into<Vec<f64>>,
    {
        Self::new_select_real().values(
            values
                .into()
                .into_iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>(),
        )
    }

    pub fn new_select_real_with_file<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self::new_select_real().file(path)
    }

    fn new_select_string() -> Self {
        Self::new(GeneratorType::SelectString)
    }

    pub fn new_select_string_with_values<V>(values: V) -> Self
    where
        V: Into<Vec<String>>,
    {
        Self::new_select_string().values(values)
    }

    pub fn new_select_string_with_file<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self::new_select_string().file(path)
    }

    pub fn new_dist_int_uniform<M>(parameters: M) -> Self
    where
        M: Into<BTreeMap<String, DataValue>>,
    {
        Self::new(GeneratorType::DistIntUniform).parameters(parameters.into())
    }

    pub fn new_dist_real_uniform<M>(parameters: M) -> Self
    where
        M: Into<BTreeMap<String, DataValue>>,
    {
        Self::new(GeneratorType::DistRealUniform).parameters(parameters.into())
    }

    pub fn new_real_normal<M>(parameters: M) -> Self
    where
        M: Into<BTreeMap<String, DataValue>>,
    {
        Self::new(GeneratorType::DistRealNormal).parameters(parameters.into())
    }

    pub fn new_when<S, V>(case_blocks: V) -> Self
    where
        S: Into<String>,
        V: Into<Vec<(S, GeneratorBuilder)>>,
    {
        let _case_blocks: Vec<(S, GeneratorBuilder)> = case_blocks.into();
        let builders: Vec<GeneratorBuilder> = _case_blocks
            .into_iter()
            .map(|(condition, block)| block.condition(condition))
            .collect();

        Self::new(GeneratorType::When).children(builders)
    }

    pub fn new_date_time<DT>(range: Option<ValueBound<DT>>) -> Self
    where
        DT: Into<NaiveDateTime>,
    {
        let mut this = Self::new(GeneratorType::DateTime);
        if let Some(range) = range {
            this = this.range(
                range.convert_with(|v| Into::<NaiveDateTime>::into(v).format("%F %T").to_string()),
            );
        }

        this
    }

    pub fn new_date<D>(range: Option<ValueBound<D>>) -> Self
    where
        D: Into<NaiveDate>,
    {
        let mut this = Self::new(GeneratorType::Date);
        if let Some(range) = range {
            this = this
                .range(range.convert_with(|v| Into::<NaiveDate>::into(v).format("%F").to_string()));
        }

        this
    }

    pub fn new_time<T>(range: Option<ValueBound<T>>) -> Self
    where
        T: Into<NaiveTime>,
    {
        let mut this = Self::new(GeneratorType::Time);
        if let Some(range) = range {
            this = this
                .range(range.convert_with(|v| Into::<NaiveTime>::into(v).format("%T").to_string()));
        }

        this
    }

    pub fn new_increment_id(range: Option<ValueBound<u64>>) -> Self {
        let mut this = Self::new(GeneratorType::IncrementId);
        if let Some(range) = range {
            this = this.range(range.convert_with(|v| format!("{}", v)));
        }

        this
    }

    //
    // build parameter functions following:
    //

    pub fn nullable(mut self) -> Self {
        self.nullable = Nullable::new_nullable();
        self
    }

    pub fn with_key<S>(mut self, key: S) -> Self
    where
        S: Into<String>,
    {
        self.key = Some(key.into());
        self
    }

    fn range<S>(mut self, range: ValueBound<S>) -> Self
    where
        S: Into<String>,
    {
        self.range = Some(range.convert_into());
        self
    }

    fn condition<S>(mut self, condition: S) -> Self
    where
        S: Into<String>,
    {
        self.condition = Some(condition.into());
        self
    }

    fn children<V>(mut self, children: V) -> Self
    where
        V: Into<Vec<GeneratorBuilder>>,
    {
        self.children = Some(children.into());
        self
    }

    fn chars<S>(mut self, chars: S) -> Self
    where
        S: Into<String>,
    {
        self.chars = Some(chars.into());
        self
    }

    fn values<V>(mut self, values: V) -> Self
    where
        V: Into<Vec<String>>,
    {
        self.values = Some(values.into());
        self
    }

    fn file<P>(mut self, path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.file = Some(path.into());
        self
    }

    fn separator<S>(mut self, separator: S) -> Self
    where
        S: Into<String>,
    {
        self.separator = Some(separator.into());
        self
    }

    fn format<S>(mut self, format: S) -> Self
    where
        S: Into<String>,
    {
        self.format = Some(format.into());
        self
    }

    fn parameters<M>(mut self, parameters: M) -> Self
    where
        M: Into<BTreeMap<String, DataValue>>,
    {
        self.parameters = Some(parameters.into());
        self
    }
}
