use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::builder::{Nullable, ValueBound, ValueStep};
use crate::error::BuildError;
use crate::generator::build_string::{DuplicatePermutationGenerator, FormatGenerator};
use crate::generator::distribution::NormalGenerator;
use crate::generator::evaluate::EvalGenerator;
use crate::generator::primitive::{
    AlwaysNullGenerator, BoolGenerator, DateGenerator, DateTimeGenerator, IncrementIdGenerator,
    IntGenerator, RealGenerator, TimeGenerator,
};
use crate::generator::random_children::CaseWhenGenerator;
use crate::generator::random_values::SelectGenerator;
use crate::generator::random_values_children::RandomizeGenerator;
use crate::generator::{Generator, Randomizer};
use crate::generator_type::GeneratorType;
use crate::value::{
    DataValue, DataValueMap, SbrdBool, SbrdDate, SbrdDateTime, SbrdInt, SbrdReal, SbrdString,
    SbrdTime,
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ParentGeneratorBuilder {
    pub(crate) key: String,
    #[serde(flatten)]
    pub(crate) builder: GeneratorBuilder,
}

impl ParentGeneratorBuilder {
    pub fn new<S>(key: S, builder: GeneratorBuilder) -> ParentGeneratorBuilder
    where
        S: Into<String>,
    {
        Self {
            key: key.into(),
            builder,
        }
    }

    pub fn split_key(self) -> (String, GeneratorBuilder) {
        let Self { key, builder } = self;
        (key, builder)
    }
}

pub type Weight = u8;
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ChildGeneratorBuilder {
    #[serde(rename = "case", skip_serializing_if = "Option::is_none")]
    pub(crate) condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) weight: Option<Weight>,
    #[serde(flatten)]
    pub(crate) builder: GeneratorBuilder,
}

impl ChildGeneratorBuilder {
    pub fn new(builder: GeneratorBuilder) -> ChildGeneratorBuilder {
        Self {
            condition: None,
            weight: None,
            builder,
        }
    }

    pub fn condition<S>(mut self, condition: S) -> Self
    where
        S: Into<String>,
    {
        self.condition = Some(condition.into());
        self
    }

    pub fn weight(mut self, weight: Weight) -> Self {
        self.weight = Some(weight);
        self
    }
}

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
    pub(crate) range: Option<ValueBound<DataValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) increment: Option<ValueStep<DataValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) children: Option<Vec<ChildGeneratorBuilder>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) chars: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) values: Option<Vec<DataValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filepath: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) separator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) script: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parameters: Option<DataValueMap<String>>,
}

macro_rules! build_generator {
    ($builder: expr,$R:ty, $builder_type: ty) => {{
        let generator: $builder_type = Generator::<$R>::create($builder)?;
        Ok(Box::new(generator))
    }};
}

//
// building
//
impl GeneratorBuilder {
    pub fn build<R: 'static + Randomizer + ?Sized>(
        self,
    ) -> Result<Box<dyn Generator<R>>, BuildError> {
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
            GeneratorType::Randomize => build_generator!(self, R, RandomizeGenerator<R>),
            GeneratorType::DuplicatePermutation => {
                build_generator!(self, R, DuplicatePermutationGenerator<R>)
            }
            GeneratorType::CaseWhen => build_generator!(self, R, CaseWhenGenerator<R>),
            GeneratorType::SelectInt => build_generator!(self, R, SelectGenerator<SbrdInt>),
            GeneratorType::SelectReal => build_generator!(self, R, SelectGenerator<SbrdReal>),
            GeneratorType::SelectString => build_generator!(self, R, SelectGenerator<SbrdString>),
            GeneratorType::DistNormal => build_generator!(self, R, NormalGenerator),
        }
    }

    pub fn into_parent<S>(self, key: S) -> ParentGeneratorBuilder
    where
        S: Into<String>,
    {
        ParentGeneratorBuilder::new(key, self)
    }

    pub fn into_child(self) -> ChildGeneratorBuilder {
        ChildGeneratorBuilder::new(self)
    }
}

//
// constructor functions following:
//
impl GeneratorBuilder {
    fn new(generator_type: GeneratorType) -> Self {
        Self {
            generator_type,
            nullable: Nullable::new_required(),
            range: None,
            increment: None,
            filepath: None,
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
            this = this.range(range.convert_into());
        }

        this
    }

    pub fn new_real(range: Option<ValueBound<SbrdReal>>) -> Self {
        let mut this = Self::new(GeneratorType::Real);
        if let Some(range) = range {
            this = this.range(range.convert_into());
        }

        this
    }

    pub fn new_bool() -> Self {
        Self::new(GeneratorType::Bool)
    }

    pub fn new_date_time(range: Option<ValueBound<SbrdDateTime>>, format: Option<String>) -> Self {
        let mut this = Self::new(GeneratorType::DateTime);
        if let Some(range) = range {
            this = this.range(range.convert_with(|v| {
                Into::<SbrdDateTime>::into(v)
                    .format("%F %T")
                    .to_string()
                    .into()
            }));
        }

        if let Some(_format) = format {
            this = this.format(_format);
        }

        this
    }

    pub fn new_date(range: Option<ValueBound<SbrdDate>>, format: Option<String>) -> Self {
        let mut this = Self::new(GeneratorType::Date);
        if let Some(range) = range {
            this = this.range(
                range.convert_with(|v| Into::<SbrdDate>::into(v).format("%F").to_string().into()),
            );
        }

        if let Some(_format) = format {
            this = this.format(_format);
        }

        this
    }

    pub fn new_time(range: Option<ValueBound<SbrdTime>>, format: Option<String>) -> Self {
        let mut this = Self::new(GeneratorType::Time);
        if let Some(range) = range {
            this = this.range(
                range.convert_with(|v| Into::<SbrdTime>::into(v).format("%T").to_string().into()),
            );
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
            this = this.increment(_increment.convert_with(DataValue::from))
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

    fn new_randomize() -> Self {
        Self::new(GeneratorType::Randomize)
    }

    pub fn new_randomize_with_children(children: Vec<ChildGeneratorBuilder>) -> Self {
        Self::new_randomize().children(children)
    }

    pub fn new_randomize_with_select_list(
        chars: Option<String>,
        values: Option<Vec<String>>,
        filepath: Option<PathBuf>,
    ) -> Self {
        let mut this = Self::new_randomize();
        if chars.is_none() && values.is_none() && filepath.is_none() {
            // default setting
            this = this.values(Vec::new());
        } else {
            if let Some(chars) = chars {
                this = this.chars(chars);
            }
            if let Some(values) = values {
                this = this.values(values.into_iter().map(|v| v.into()).collect());
            }
            if let Some(filepath) = filepath {
                this = this.filepath(filepath);
            }
        }

        this
    }

    fn new_duplicate_permutation<S>(range: Option<ValueBound<SbrdInt>>, separator: S) -> Self
    where
        S: Into<String>,
    {
        let mut this = Self::new(GeneratorType::DuplicatePermutation).separator(separator);
        if let Some(range) = range {
            this = this.range(range.convert_into());
        }

        this
    }

    pub fn new_duplicate_permutation_with_children<S>(
        range: Option<ValueBound<SbrdInt>>,
        separator: S,
        children: Vec<ChildGeneratorBuilder>,
    ) -> Self
    where
        S: Into<String>,
    {
        Self::new_duplicate_permutation(range, separator).children(children)
    }

    pub fn new_duplicate_permutation_with_select_list<S>(
        range: Option<ValueBound<SbrdInt>>,
        separator: S,
        chars: Option<String>,
        values: Option<Vec<String>>,
        filepath: Option<PathBuf>,
    ) -> Self
    where
        S: Into<String>,
    {
        let mut this = Self::new_duplicate_permutation(range, separator);
        if chars.is_none() && values.is_none() && filepath.is_none() {
            // default setting
            this = this.values(Vec::new());
        } else {
            if let Some(chars) = chars {
                this = this.chars(chars);
            }
            if let Some(values) = values {
                this = this.values(values.into_iter().map(|v| v.into()).collect());
            }
            if let Some(filepath) = filepath {
                this = this.filepath(filepath);
            }
        }

        this
    }

    pub fn new_case_when(children: Vec<ChildGeneratorBuilder>) -> Self {
        Self::new(GeneratorType::CaseWhen).children(children)
    }

    pub fn new_select_int(
        chars: Option<String>,
        values: Option<Vec<SbrdInt>>,
        filepath: Option<PathBuf>,
    ) -> Self {
        let mut this = Self::new(GeneratorType::SelectInt);
        if chars.is_none() && values.is_none() && filepath.is_none() {
            // default setting
            this = this.values(Vec::new());
        } else {
            if let Some(chars) = chars {
                this = this.chars(chars);
            }
            if let Some(values) = values {
                this = this.values(values.into_iter().map(|v| v.into()).collect());
            }
            if let Some(filepath) = filepath {
                this = this.filepath(filepath);
            }
        }

        this
    }

    pub fn new_select_real(
        chars: Option<String>,
        values: Option<Vec<SbrdReal>>,
        filepath: Option<PathBuf>,
    ) -> Self {
        let mut this = Self::new(GeneratorType::SelectReal);
        if chars.is_none() && values.is_none() && filepath.is_none() {
            // default setting
            this = this.values(Vec::new());
        } else {
            if let Some(chars) = chars {
                this = this.chars(chars);
            }
            if let Some(values) = values {
                this = this.values(values.into_iter().map(|v| v.into()).collect());
            }
            if let Some(filepath) = filepath {
                this = this.filepath(filepath);
            }
        }

        this
    }

    pub fn new_select_string(
        chars: Option<String>,
        values: Option<Vec<SbrdString>>,
        filepath: Option<PathBuf>,
    ) -> Self {
        let mut this = Self::new(GeneratorType::SelectString);
        if chars.is_none() && values.is_none() && filepath.is_none() {
            // default setting
            this = this.values(Vec::new());
        } else {
            if let Some(chars) = chars {
                this = this.chars(chars);
            }
            if let Some(values) = values {
                this = this.values(values.into_iter().map(|v| v.into()).collect());
            }
            if let Some(filepath) = filepath {
                this = this.filepath(filepath);
            }
        }

        this
    }

    pub fn new_dist_normal(mean: SbrdReal, std_dev: SbrdReal) -> Self {
        let mut parameters = DataValueMap::new();
        parameters.insert(NormalGenerator::MEAN.to_string(), mean.into());
        parameters.insert(NormalGenerator::STD_DEV.to_string(), std_dev.into());
        Self::new(GeneratorType::DistNormal).parameters(parameters)
    }
}

//
// build parameter functions following:
//
impl GeneratorBuilder {
    pub fn nullable(mut self) -> Self {
        self.nullable = Nullable::new_nullable();
        self
    }

    fn range(mut self, range: ValueBound<DataValue>) -> Self {
        self.range = Some(range);
        self
    }

    fn increment(mut self, increment: ValueStep<DataValue>) -> Self {
        self.increment = Some(increment);
        self
    }

    fn children(mut self, children: Vec<ChildGeneratorBuilder>) -> Self {
        self.children = Some(children);
        self
    }

    fn chars<S>(mut self, chars: S) -> Self
    where
        S: Into<String>,
    {
        self.chars = Some(chars.into());
        self
    }

    fn values(mut self, values: Vec<DataValue>) -> Self {
        self.values = Some(values);
        self
    }

    fn filepath<P>(mut self, filepath: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.filepath = Some(filepath.into());
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

    fn parameters(mut self, parameters: DataValueMap<String>) -> Self {
        self.parameters = Some(parameters);
        self
    }
}
