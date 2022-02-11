use std::collections::btree_map::BTreeMap;
use std::path::PathBuf;

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::bound::ValueBound;
use crate::generator_type::GeneratorType;
use crate::generators::error::CompileError;
use crate::generators::{
    AlwaysNullGenerator, BoolGenerator, DateGenerator, DateTimeGenerator, EvalGenerator,
    FormatGenerator, Generator, IncrementIdGenerator, IntGenerator, RealGenerator, TimeGenerator,
};
use crate::value::DataValue;
use crate::{Nullable, SbrdBool, SbrdDate, SbrdDateTime, SbrdInt, SbrdReal, SbrdTime, ValueStep};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct GeneratorBuilder {
    #[serde(rename = "type")]
    pub(crate) generator_type: GeneratorType,
    #[serde(
        skip_serializing_if = "Nullable::is_required",
        default = "Nullable::new_required"
    )]
    pub(crate) nullable: Nullable,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) range: Option<ValueBound<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) increment: Option<ValueStep<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) children: Option<Vec<GeneratorBuilder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) chars: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) file: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) separator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parameters: Option<BTreeMap<String, DataValue>>,
}

macro_rules! build_generator {
    ($builder: expr,$R:ty, $builder_type: ty) => {{
        let generator: $builder_type = Generator::<$R>::create($builder)?;
        Ok(Box::new(generator))
    }};
}

// create builder impl
impl GeneratorBuilder {
    pub fn build<R: Rng + ?Sized>(self) -> Result<Box<dyn Generator<R>>, CompileError> {
        match self.generator_type {
            GeneratorType::Int => build_generator!(self, R, IntGenerator),
            GeneratorType::Real => build_generator!(self, R, RealGenerator),
            GeneratorType::Bool => build_generator!(self, R, BoolGenerator),
            GeneratorType::DateTime => build_generator!(self, R, DateTimeGenerator),
            GeneratorType::Date => build_generator!(self, R, DateGenerator),
            GeneratorType::Time => build_generator!(self, R, TimeGenerator),
            GeneratorType::AlwaysNull => build_generator!(self, R, AlwaysNullGenerator),
            GeneratorType::IncrementId => build_generator!(self, R, IncrementIdGenerator),
            GeneratorType::EvalInt => build_generator!(self, R, EvalGenerator<SbrdInt>),
            GeneratorType::EvalReal => build_generator!(self, R, EvalGenerator<SbrdReal>),
            GeneratorType::EvalBool => build_generator!(self, R, EvalGenerator<SbrdBool>),
            GeneratorType::Format => build_generator!(self, R, FormatGenerator),
            GeneratorType::DuplicatePermutation => unimplemented!(),
            GeneratorType::SelectInt => unimplemented!(),
            GeneratorType::SelectReal => unimplemented!(),
            GeneratorType::SelectString => unimplemented!(),
            GeneratorType::DistIntUniform => unimplemented!(),
            GeneratorType::DistRealUniform => unimplemented!(),
            GeneratorType::DistRealNormal => unimplemented!(),
            GeneratorType::When => unimplemented!(),
        }
    }

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
            increment: None,
            file: None,
            separator: None,
            values: None,
            format: None,
            script: None,
            chars: None,
            parameters: None,
            children: None,
        }
    }

    pub fn new_int(range: Option<ValueBound<SbrdInt>>) -> Self {
        let mut this = Self::new(GeneratorType::Int);
        if let Some(range) = range {
            this = this.range(range.convert_with(|v| format!("{}", v)));
        }

        this
    }

    pub fn new_real(range: Option<ValueBound<SbrdReal>>) -> Self {
        let mut this = Self::new(GeneratorType::Real);
        if let Some(range) = range {
            this = this.range(range.convert_with(|v| format!("{}", v)));
        }

        this
    }

    pub fn new_bool() -> Self {
        Self::new(GeneratorType::Bool)
    }

    pub fn new_date_time<DT, S>(range: Option<ValueBound<DT>>, format: Option<S>) -> Self
    where
        DT: Into<SbrdDateTime>,
        S: Into<String>,
    {
        let mut this = Self::new(GeneratorType::DateTime);
        if let Some(range) = range {
            this = this.range(
                range.convert_with(|v| Into::<SbrdDateTime>::into(v).format("%F %T").to_string()),
            );
        }

        if let Some(_format) = format {
            this = this.format(_format);
        }

        this
    }

    pub fn new_date<D, S>(range: Option<ValueBound<D>>, format: Option<S>) -> Self
    where
        D: Into<SbrdDate>,
        S: Into<String>,
    {
        let mut this = Self::new(GeneratorType::Date);
        if let Some(range) = range {
            this = this
                .range(range.convert_with(|v| Into::<SbrdDate>::into(v).format("%F").to_string()));
        }

        if let Some(_format) = format {
            this = this.format(_format);
        }

        this
    }

    pub fn new_time<T, S>(range: Option<ValueBound<T>>, format: Option<S>) -> Self
    where
        T: Into<SbrdTime>,
        S: Into<String>,
    {
        let mut this = Self::new(GeneratorType::Time);
        if let Some(range) = range {
            this = this
                .range(range.convert_with(|v| Into::<SbrdTime>::into(v).format("%T").to_string()));
        }

        if let Some(_format) = format {
            this = this.format(_format);
        }

        this
    }

    pub fn new_always_null() -> Self {
        Self::new(GeneratorType::AlwaysNull)
    }

    pub fn new_increment_id(increment: Option<ValueStep<SbrdInt>>) -> Self {
        let mut this = Self::new(GeneratorType::IncrementId);

        if let Some(_increment) = increment {
            this = this.increment(_increment.convert_with(|v| v.to_string()))
        }

        this
    }

    pub fn new_eval_int<S>(script: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::EvalInt).script(script)
    }

    pub fn new_eval_real<S>(script: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::EvalReal).script(script)
    }

    pub fn new_eval_bool<S>(script: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::EvalBool).script(script)
    }

    pub fn new_format<S>(format: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::Format).format(format)
    }

    fn new_duplicate_permutation<S>(range: Option<ValueBound<SbrdInt>>, separator: S) -> Self
    where
        S: Into<String>,
    {
        let mut this = Self::new(GeneratorType::DuplicatePermutation).separator(separator);
        if let Some(range) = range {
            this = this.range(range.convert_with(|v| format!("{}", v)));
        }

        this
    }

    pub fn new_duplicate_permutation_with_chars<S1, S2>(
        range: Option<ValueBound<SbrdInt>>,
        separator: S1,
        chars: S2,
    ) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self::new_duplicate_permutation(range, separator).chars(chars)
    }

    pub fn new_duplicate_permutation_with_children<S, V>(
        range: Option<ValueBound<SbrdInt>>,
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
        V: Into<Vec<SbrdInt>>,
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
        V: Into<Vec<SbrdReal>>,
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
        Self::new(GeneratorType::DistIntUniform).parameters(parameters)
    }

    pub fn new_dist_real_uniform<M>(parameters: M) -> Self
    where
        M: Into<BTreeMap<String, DataValue>>,
    {
        Self::new(GeneratorType::DistRealUniform).parameters(parameters)
    }

    pub fn new_real_normal<M>(parameters: M) -> Self
    where
        M: Into<BTreeMap<String, DataValue>>,
    {
        Self::new(GeneratorType::DistRealNormal).parameters(parameters)
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

    fn increment<S>(mut self, increment: ValueStep<S>) -> Self
    where
        S: Into<String>,
    {
        self.increment = Some(increment.convert_into());
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

    fn script<S>(mut self, script: S) -> Self
    where
        S: Into<String>,
    {
        self.script = Some(script.into());
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
