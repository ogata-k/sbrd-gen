//! Module for builder

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
use crate::generator::random_values::{
    GetValueAtGenerator, GetValueIndexGenerator, SelectGenerator,
};
use crate::generator::random_values_children::RandomizeGenerator;
use crate::generator::{Generator, Randomizer};
use crate::generator_type::GeneratorType;
use crate::value::{
    DataValue, DataValueMap, SbrdBool, SbrdDate, SbrdDateTime, SbrdInt, SbrdReal, SbrdString,
    SbrdTime, DATE_DEFAULT_FORMAT, DATE_TIME_DEFAULT_FORMAT, TIME_DEFAULT_FORMAT,
};

/// Generator Builder used in [`SchemeBuilder`] as Generator Builder.
///
/// [`SchemeBuilder`]: ../schema/struct.SchemaBuilder.html
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ParentGeneratorBuilder {
    pub(crate) key: String,
    #[serde(flatten)]
    pub(crate) builder: GeneratorBuilder,
}

impl ParentGeneratorBuilder {
    /// Create from [`GeneratorBuilder`]
    ///
    /// [`GeneratorBuilder`]: ./struct.GeneratorBuilder.html
    fn new<S>(key: S, builder: GeneratorBuilder) -> ParentGeneratorBuilder
    where
        S: Into<String>,
    {
        Self {
            key: key.into(),
            builder,
        }
    }

    /// Split to `key` and [`GeneratorBuilder`]
    ///
    /// [`GeneratorBuilder`]: ./struct.GeneratorBuilder.html
    pub fn split_key(self) -> (String, GeneratorBuilder) {
        let Self { key, builder } = self;
        (key, builder)
    }
}

/// Alias of Weight
pub type Weight = u8;

/// Generator Builder used in [`SchemeBuilder`] as Generator Builder's Children.
///
/// [`SchemeBuilder`]: ../schema/struct.SchemaBuilder.html
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ChildGeneratorBuilder {
    /// Child Generator's `condition` option
    #[serde(rename = "case", skip_serializing_if = "Option::is_none")]
    pub(crate) condition: Option<String>,

    /// Child Generator's `weight` option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) weight: Option<Weight>,

    /// Child Generator's base builder
    #[serde(flatten)]
    pub(crate) builder: GeneratorBuilder,
}

impl ChildGeneratorBuilder {
    /// Create from [`GeneratorBuilder`]
    ///
    /// [`GeneratorBuilder`]: ./struct.GeneratorBuilder.html
    fn new(builder: GeneratorBuilder) -> ChildGeneratorBuilder {
        Self {
            condition: None,
            weight: None,
            builder,
        }
    }

    /// Set `condition` condition
    pub fn condition<S>(mut self, condition: S) -> Self
    where
        S: Into<String>,
    {
        self.condition = Some(condition.into());
        self
    }

    /// Set `weight` condition
    pub fn weight(mut self, weight: Weight) -> Self {
        self.weight = Some(weight);
        self
    }
}

/// Base Generator Builder
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct GeneratorBuilder {
    /// Generator's `type` parameter
    ///
    /// This is a type of the generator.
    #[serde(rename = "type")]
    pub(crate) generator_type: GeneratorType,
    #[serde(
        skip_serializing_if = "Nullable::is_required",
        default = "Nullable::new_required"
    )]

    /// Generator's `nullable` status
    ///
    /// This is a nullable flag for the generator.
    pub(crate) nullable: Nullable,

    /// Generator's `range` option
    ///
    /// This is a range for the generated value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) range: Option<ValueBound<DataValue>>,

    /// Generator's `increment` option
    ///
    /// This is a data of each step values from initial for the generated value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) increment: Option<ValueStep<DataValue>>,

    /// Generator's `children` option
    ///
    /// The generator pick out the child from this children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) children: Option<Vec<ChildGeneratorBuilder>>,

    /// Generator's `characters` option
    ///
    /// The generator pick out the character from this characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) chars: Option<String>,

    /// Generator's `values` option
    ///
    /// The generator pick out the value from this values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) values: Option<Vec<DataValue>>,

    /// Generator's `filepath` option
    ///
    /// The generator pick out the value from the lines in the file at the filepath.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filepath: Option<PathBuf>,

    /// Generator's `separator` option
    ///
    /// This separator use as glue to join the generated values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) separator: Option<String>,

    /// Generator's `format` option
    ///
    /// This is a format for the generated value.
    /// Evaluate by [`Evaluator`] as String.
    ///
    /// [`Evaluator`]: ../eval/struct.Evaluator.html
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) format: Option<String>,

    /// Generator's `script` option
    ///
    /// This is a script for the generated value.
    /// Evaluate by [`Evaluator`] as not String.
    ///
    /// [`Evaluator`]: ../eval/struct.Evaluator.html
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) script: Option<String>,

    /// Generator's `parameters` option
    ///
    /// This is a parameter data set for a Distribution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) parameters: Option<DataValueMap<String>>,
}

/// Helper for build generator.
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
    /// Build generator as the type
    pub fn build<R: Randomizer + ?Sized>(self) -> Result<Box<dyn Generator<R>>, BuildError> {
        match self.generator_type {
            // build string
            GeneratorType::DuplicatePermutation => {
                build_generator!(self, R, DuplicatePermutationGenerator<R>)
            }
            GeneratorType::Format => build_generator!(self, R, FormatGenerator),

            // distribution
            GeneratorType::DistNormal => build_generator!(self, R, NormalGenerator),

            // evaluate
            GeneratorType::EvalInt => build_generator!(self, R, EvalGenerator<SbrdInt>),
            GeneratorType::EvalReal => build_generator!(self, R, EvalGenerator<SbrdReal>),
            GeneratorType::EvalBool => build_generator!(self, R, EvalGenerator<SbrdBool>),
            GeneratorType::EvalString => build_generator!(self, R, EvalGenerator<SbrdString>),

            // primitive
            GeneratorType::Int => build_generator!(self, R, IntGenerator),
            GeneratorType::Real => build_generator!(self, R, RealGenerator),
            GeneratorType::Bool => build_generator!(self, R, BoolGenerator),
            GeneratorType::DateTime => build_generator!(self, R, DateTimeGenerator),
            GeneratorType::Date => build_generator!(self, R, DateGenerator),
            GeneratorType::Time => build_generator!(self, R, TimeGenerator),
            GeneratorType::AlwaysNull => build_generator!(self, R, AlwaysNullGenerator),
            GeneratorType::IncrementId => build_generator!(self, R, IncrementIdGenerator),

            // randomize children
            GeneratorType::CaseWhen => build_generator!(self, R, CaseWhenGenerator<R>),

            // randomize values
            GeneratorType::SelectInt => build_generator!(self, R, SelectGenerator<SbrdInt>),
            GeneratorType::SelectReal => build_generator!(self, R, SelectGenerator<SbrdReal>),
            GeneratorType::SelectString => build_generator!(self, R, SelectGenerator<SbrdString>),
            GeneratorType::GetIntValueAt => build_generator!(self, R, GetValueAtGenerator<SbrdInt>),
            GeneratorType::GetRealValueAt => {
                build_generator!(self, R, GetValueAtGenerator<SbrdReal>)
            }
            GeneratorType::GetStringValueAt => {
                build_generator!(self, R, GetValueAtGenerator<SbrdString>)
            }
            GeneratorType::GetValueIndex => {
                build_generator!(self, R, GetValueIndexGenerator)
            }

            // random values and children
            GeneratorType::Randomize => build_generator!(self, R, RandomizeGenerator<R>),
        }
    }

    /// Convert this builder to parent builder
    pub fn into_parent<S>(self, key: S) -> ParentGeneratorBuilder
    where
        S: Into<String>,
    {
        ParentGeneratorBuilder::new(key, self)
    }

    /// Convert this builder to child
    pub fn into_child(self) -> ChildGeneratorBuilder {
        ChildGeneratorBuilder::new(self)
    }
}

//
// constructor functions following:
//
impl GeneratorBuilder {
    /// Create generate builder as the type.
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

    //
    // build string
    //

    /// Create builder for [`DuplicatePermutationGenerator`]
    ///
    /// [`DuplicatePermutationGenerator`]: ../generator/build_string/duplicate_permutation/struct.DuplicatePermutationGenerator.html
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

    /// Create builder for [`DuplicatePermutationGenerator`] as generator with generate from children
    ///
    /// [`DuplicatePermutationGenerator`]: ../generator/build_string/duplicate_permutation/struct.DuplicatePermutationGenerator.html
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

    /// Create builder for [`DuplicatePermutationGenerator`] as generator with generate from chars, values, file
    ///
    /// [`DuplicatePermutationGenerator`]: ../generator/build_string/duplicate_permutation/struct.DuplicatePermutationGenerator.html
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

    /// Create builder for [`FormatGenerator`]
    ///
    /// [`FormatGenerator`]: ../generator/build_string/format_generator/struct.FormatGenerator.html
    pub fn new_format<S>(format: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::Format).format(format)
    }

    //
    // distribution
    //

    /// Create builder for [`NormalGenerator`]
    ///
    /// [`NormalGenerator`]: ../generator/distribution/normal_generator/struct.NormalGenerator.html
    pub fn new_dist_normal(mean: SbrdReal, std_dev: SbrdReal) -> Self {
        let mut parameters = DataValueMap::new();
        parameters.insert(NormalGenerator::MEAN.to_string(), mean.into());
        parameters.insert(NormalGenerator::STD_DEV.to_string(), std_dev.into());
        Self::new(GeneratorType::DistNormal).parameters(parameters)
    }

    //
    // evaluate
    //

    /// Create builder for [`EvalGenerator`] with type [`SbrdInt`]
    ///
    /// [`EvalGenerator`]: ../generator/evaluate/eval_generator/struct.EvalGenerator.html
    /// [`SbrdInt`]: ../value/type.SbrdInt.html
    pub fn new_eval_int<S>(script: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::EvalInt).script(script)
    }

    /// Create builder for [`EvalGenerator`] with type [`SbrdReal`]
    ///
    /// [`EvalGenerator`]: ../generator/evaluate/eval_generator/struct.EvalGenerator.html
    /// [`SbrdReal`]: ../value/type.SbrdReal.html
    pub fn new_eval_real<S>(script: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::EvalReal).script(script)
    }

    /// Create builder for [`EvalGenerator`] with type [`SbrdBool`]
    ///
    /// [`EvalGenerator`]: ../generator/evaluate/eval_generator/struct.EvalGenerator.html
    /// [`SbrdBool`]: ../value/type.SbrdBool.html
    pub fn new_eval_bool<S>(script: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::EvalBool).script(script)
    }

    /// Create builder for [`EvalGenerator`] with type [`SbrdString`]
    ///
    /// [`EvalGenerator`]: ../generator/evaluate/eval_generator/struct.EvalGenerator.html
    /// [`SbrdString`]: ../value/type.SbrdString.html
    pub fn new_eval_string<S>(script: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::EvalString).script(script)
    }

    //
    // primitive
    //

    /// Create builder for [`IntGenerator`]
    ///
    /// [`IntGenerator`]: ../generator/primitive/int_generator/struct.IntGenerator.html
    pub fn new_int(range: Option<ValueBound<SbrdInt>>) -> Self {
        let mut this = Self::new(GeneratorType::Int);
        if let Some(range) = range {
            this = this.range(range.convert_into());
        }

        this
    }

    /// Create builder for [`RealGenerator`]
    ///
    /// [`RealGenerator`]: ../generator/primitive/real_generator/struct.RealGenerator.html
    pub fn new_real(range: Option<ValueBound<SbrdReal>>) -> Self {
        let mut this = Self::new(GeneratorType::Real);
        if let Some(range) = range {
            this = this.range(range.convert_into());
        }

        this
    }

    /// Create builder for [`BoolGenerator`]
    ///
    /// [`BoolGenerator`]: ../generator/primitive/bool_generator/struct.BoolGenerator.html
    pub fn new_bool() -> Self {
        Self::new(GeneratorType::Bool)
    }

    /// Create builder for [`DateTimeGenerator`].
    /// See [`format::strftime` module] for more information on `format` option.
    /// The default for `format` and the format when parsing is [`DATE_TIME_DEFAULT_FORMAT`].
    ///
    /// [`DateTimeGenerator`]: ../generator/primitive/date_time_generator/struct.DateTimeGenerator.html
    /// [`format::strftime` module]: https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers
    /// [`DATE_TIME_DEFAULT_FORMAT`]: ../value/constant.DATE_TIME_DEFAULT_FORMAT.html
    pub fn new_date_time(range: Option<ValueBound<SbrdDateTime>>, format: Option<String>) -> Self {
        let mut this = Self::new(GeneratorType::DateTime);
        if let Some(range) = range {
            this = this.range(range.convert_with(|v| {
                Into::<SbrdDateTime>::into(v)
                    .format(DATE_TIME_DEFAULT_FORMAT)
                    .to_string()
                    .into()
            }));
        }

        if let Some(_format) = format {
            this = this.format(_format);
        }

        this
    }

    /// Create builder for [`DateGenerator`]
    /// See [`format::strftime` module] for more information on `format` option.
    /// The default for `format` and the format when parsing is [`DATE_DEFAULT_FORMAT`].
    ///
    /// [`DateGenerator`]: ../generator/primitive/date_generator/struct.DateGenerator.html
    /// [`format::strftime` module]: https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers
    /// [`DATE_DEFAULT_FORMAT`]: ../value/constant.DATE_DEFAULT_FORMAT.html
    pub fn new_date(range: Option<ValueBound<SbrdDate>>, format: Option<String>) -> Self {
        let mut this = Self::new(GeneratorType::Date);
        if let Some(range) = range {
            this = this.range(range.convert_with(|v| {
                Into::<SbrdDate>::into(v)
                    .format(DATE_DEFAULT_FORMAT)
                    .to_string()
                    .into()
            }));
        }

        if let Some(_format) = format {
            this = this.format(_format);
        }

        this
    }

    /// Create builder for [`TimeGenerator`]
    /// See [`format::strftime` module] for more information on `format` option.
    /// The default for `format` and the format when parsing is [`TIME_DEFAULT_FORMAT`].
    ///
    /// [`TimeGenerator`]: ../generator/primitive/time_generator/struct.TimeGenerator.html
    /// [`format::strftime` module]: https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers
    /// [`TIME_DEFAULT_FORMAT`]: ../value/constant.TIME_DEFAULT_FORMAT.html
    pub fn new_time(range: Option<ValueBound<SbrdTime>>, format: Option<String>) -> Self {
        let mut this = Self::new(GeneratorType::Time);
        if let Some(range) = range {
            this = this.range(range.convert_with(|v| {
                Into::<SbrdTime>::into(v)
                    .format(TIME_DEFAULT_FORMAT)
                    .to_string()
                    .into()
            }));
        }

        if let Some(_format) = format {
            this = this.format(_format);
        }

        this
    }

    /// Create builder for [`AlwaysNullGenerator`]
    ///
    /// [`AlwaysNullGenerator`]: ../generator/primitive/always_null_generator/struct.AlwaysNullGenerator.html
    pub fn new_always_null() -> Self {
        Self::new(GeneratorType::AlwaysNull)
    }

    /// Create builder for [`IncrementIdGenerator`]
    ///
    /// [`IncrementIdGenerator`]: ../generator/primitive/increment_id_generator/struct.IncrementIdGenerator.html
    pub fn new_increment_id(increment: Option<ValueStep<SbrdInt>>) -> Self {
        let mut this = Self::new(GeneratorType::IncrementId);

        if let Some(_increment) = increment {
            this = this.increment(_increment.convert_with(DataValue::from))
        }

        this
    }

    //
    // randomize children
    //

    /// Create builder for [`CaseWhenGenerator`] as generator with generate from children
    ///
    /// [`CaseWhenGenerator`]: ../generator/random_children/case_when_generator/struct.CaseWhenGenerator.html
    pub fn new_case_when(children: Vec<ChildGeneratorBuilder>) -> Self {
        Self::new(GeneratorType::CaseWhen).children(children)
    }

    //
    // randomize values
    //

    /// Create builder for [`SelectGenerator`] with type [`SbrdInt`]
    ///
    /// [`SelectGenerator`]: ../generator/random_values/select_generator/struct.SelectGenerator.html
    /// [`SbrdInt`]: ../value/type.SbrdInt.html
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

    /// Create builder for [`SelectGenerator`] with type [`SbrdReal`]
    ///
    /// [`SelectGenerator`]: ../generator/random_values/select_generator/struct.SelectGenerator.html
    /// [`SbrdReal`]: ../value/type.SbrdReal.html
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

    /// Create builder for [`SelectGenerator`] with type [`SbrdString`]
    ///
    /// [`SelectGenerator`]: ../generator/random_values/select_generator/struct.SelectGenerator.html
    /// [`SbrdString`]: ../value/type.SbrdString.html
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

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdInt`]
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdInt`]: ../value/type.SbrdInt.html
    fn new_get_int_value_at<S>(script: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::GetIntValueAt).script(script)
    }

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdInt`] from chars
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdInt`]: ../value/type.SbrdInt.html
    pub fn new_get_int_value_at_from_chars<S1, S2>(script: S1, chars: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self::new_get_int_value_at(script).chars(chars)
    }

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdInt`] from values
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdInt`]: ../value/type.SbrdInt.html
    pub fn new_get_int_value_at_from_values<S>(script: S, values: Vec<SbrdInt>) -> Self
    where
        S: Into<String>,
    {
        Self::new_get_int_value_at(script).values(values.into_iter().map(|v| v.into()).collect())
    }

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdInt`] from file
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdInt`]: ../value/type.SbrdInt.html
    pub fn new_get_int_value_at_from_file<S, P>(script: S, filepath: P) -> Self
    where
        S: Into<String>,
        P: Into<PathBuf>,
    {
        Self::new_get_int_value_at(script).filepath(filepath)
    }

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdReal`]
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdReal`]: ../value/type.SbrdReal.html
    fn new_get_real_value_at<S>(script: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::GetRealValueAt).script(script)
    }

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdReal`] from chars
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdReal`]: ../value/type.SbrdReal.html
    pub fn new_get_real_value_at_from_chars<S1, S2>(script: S1, chars: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self::new_get_real_value_at(script).chars(chars)
    }

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdReal`] from values
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdReal`]: ../value/type.SbrdReal.html
    pub fn new_get_real_value_at_from_values<S>(script: S, values: Vec<SbrdReal>) -> Self
    where
        S: Into<String>,
    {
        Self::new_get_real_value_at(script).values(values.into_iter().map(|v| v.into()).collect())
    }

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdReal`] from file
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdReal`]: ../value/type.SbrdReal.html
    pub fn new_get_real_value_at_from_file<S, P>(script: S, filepath: P) -> Self
    where
        S: Into<String>,
        P: Into<PathBuf>,
    {
        Self::new_get_real_value_at(script).filepath(filepath)
    }

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdString`]
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdString`]: ../value/type.SbrdString.html
    fn new_get_string_value_at<S>(script: S) -> Self
    where
        S: Into<String>,
    {
        Self::new(GeneratorType::GetStringValueAt).script(script)
    }

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdString`] from chars
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdString`]: ../value/type.SbrdString.html
    pub fn new_get_string_value_at_from_chars<S1, S2>(script: S1, chars: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self::new_get_string_value_at(script).chars(chars)
    }

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdString`] from values
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdString`]: ../value/type.SbrdString.html
    pub fn new_get_string_value_at_from_values<S>(script: S, values: Vec<SbrdString>) -> Self
    where
        S: Into<String>,
    {
        Self::new_get_string_value_at(script).values(values.into_iter().map(|v| v.into()).collect())
    }

    /// Create builder for [`GetValueAtGenerator`] with type [`SbrdString`] from file
    ///
    /// [`GetValueAtGenerator`]: ../generator/random_values/select_generator/struct.GetValueAtGenerator.html
    /// [`SbrdString`]: ../value/type.SbrdString.html
    pub fn new_get_string_value_at_from_file<S, P>(script: S, filepath: P) -> Self
    where
        S: Into<String>,
        P: Into<PathBuf>,
    {
        Self::new_get_string_value_at(script).filepath(filepath)
    }

    /// Create builder for [`GetValueIndexGenerator`]
    ///
    /// [`GetValueIndexGenerator`]: ../generator/random_values/get_value_index_generator/struct.GetValueIndex.html
    fn new_get_value_index() -> Self {
        Self::new(GeneratorType::GetValueIndex)
    }

    /// Create builder for [`GetValueIndexGenerator`]
    ///
    /// [`GetValueIndexGenerator`]: ../generator/random_values/get_value_index_generator/struct.GetValueIndex.html
    pub fn new_get_value_index_from_chars<S>(chars: S) -> Self
    where
        S: Into<String>,
    {
        Self::new_get_value_index().chars(chars)
    }

    /// Create builder for [`GetValueIndexGenerator`]
    ///
    /// [`GetValueIndexGenerator`]: ../generator/random_values/get_value_index_generator/struct.GetValueIndex.html
    pub fn new_get_value_index_from_values(values: Vec<DataValue>) -> Self {
        Self::new_get_value_index().values(values)
    }

    /// Create builder for [`GetValueIndexGenerator`]
    ///
    /// [`GetValueIndexGenerator`]: ../generator/random_values/get_value_index_generator/struct.GetValueIndex.html
    pub fn new_get_value_index_from_file<P>(filepath: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self::new_get_value_index().filepath(filepath)
    }

    //
    // random values and children
    //

    /// Create builder for [`RandomizeGenerator`]
    ///
    /// [`RandomizeGenerator`]: ../generator/random_values_children/randomize_generator/struct.RandomizeGenerator.html
    fn new_randomize() -> Self {
        Self::new(GeneratorType::Randomize)
    }

    /// Create builder for [`RandomizeGenerator`] as generator with generate from children
    ///
    /// [`RandomizeGenerator`]: ../generator/random_values_children/randomize_generator/struct.RandomizeGenerator.html
    pub fn new_randomize_with_children(children: Vec<ChildGeneratorBuilder>) -> Self {
        Self::new_randomize().children(children)
    }

    /// Create builder for [`RandomizeGenerator`] as generator with generate from chars, values, file
    ///
    /// [`RandomizeGenerator`]: ../generator/random_values_children/randomize_generator/struct.RandomizeGenerator.html
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
}

//
// build parameter functions following:
//
impl GeneratorBuilder {
    /// Set `nullable` status to change to nullable
    pub fn nullable(mut self) -> Self {
        self.nullable = Nullable::new_nullable();
        self
    }

    //
    // setter
    //

    /// Set `range` option
    fn range(mut self, range: ValueBound<DataValue>) -> Self {
        self.range = Some(range);
        self
    }

    /// Set `range` option
    fn increment(mut self, increment: ValueStep<DataValue>) -> Self {
        self.increment = Some(increment);
        self
    }

    /// Set `children` option
    fn children(mut self, children: Vec<ChildGeneratorBuilder>) -> Self {
        self.children = Some(children);
        self
    }

    /// Set `characters` option
    fn chars<S>(mut self, chars: S) -> Self
    where
        S: Into<String>,
    {
        self.chars = Some(chars.into());
        self
    }

    /// Set `values` option
    fn values(mut self, values: Vec<DataValue>) -> Self {
        self.values = Some(values);
        self
    }

    /// Set `filepath` option
    fn filepath<P>(mut self, filepath: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.filepath = Some(filepath.into());
        self
    }

    /// Set `separator` option
    fn separator<S>(mut self, separator: S) -> Self
    where
        S: Into<String>,
    {
        self.separator = Some(separator.into());
        self
    }

    /// Set `format` option
    fn format<S>(mut self, format: S) -> Self
    where
        S: Into<String>,
    {
        self.format = Some(format.into());
        self
    }

    /// Set `script` option
    fn script<S>(mut self, script: S) -> Self
    where
        S: Into<String>,
    {
        self.script = Some(script.into());
        self
    }

    /// Set `parameters` option
    fn parameters(mut self, parameters: DataValueMap<String>) -> Self {
        self.parameters = Some(parameters);
        self
    }
}
