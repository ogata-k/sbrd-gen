use crate::builder::{GeneratorBuilder, ValueBound};
use crate::error::{BuildError, GenerateError};
use crate::eval::Evaluator;
use crate::generator::{GeneratorBase, Randomizer};
use crate::value::{DataValue, DataValueMap, SbrdDate, DATE_DEFAULT_FORMAT};
use crate::GeneratorType;
use chrono::Datelike;

/// The generator with generate [`SbrdDate`] value as [`DataValue::String`] with the format
///
/// See [`format::strftime` module] for more information on `format` option.
/// The default for `format` and the format when parsing is [`DATE_DEFAULT_FORMAT`].
///
/// [`SbrdDate`]: ../../value/type.SbrdDate.html
/// [`DataValue::String`]: ../../value/enum.DataValue.html#variant.String
/// [`format::strftime` module]: https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers
/// [`DATE_DEFAULT_FORMAT`]: ../../value/constant.DATE_DEFAULT_FORMAT.html
#[derive(Debug, PartialEq, Clone)]
pub struct DateGenerator {
    nullable: bool,
    format: String,
    range: ValueBound<SbrdDate>,
}

impl<R: Randomizer + ?Sized> GeneratorBase<R> for DateGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            range,
            format,
            ..
        } = builder;

        if generator_type != GeneratorType::Date {
            return Err(BuildError::InvalidType(generator_type));
        }

        let default_range = Self::default_range();
        let _range = match range {
            None => default_range,
            Some(r) => r
                .try_convert_with(|s| {
                    SbrdDate::parse_from_str(&s.to_parse_string(), DATE_DEFAULT_FORMAT).map_err(
                        |e| {
                            BuildError::FailParseValue(
                                s.to_parse_string(),
                                "Date".to_string(),
                                e.to_string(),
                            )
                        },
                    )
                })
                .map(|range| {
                    // If it is not specified so that it can be generated in an appropriate range, use the default boundary.
                    range.without_no_bound_from_other(default_range)
                })?,
        };
        if _range.is_empty() {
            return Err(BuildError::RangeEmpty(_range.convert_into()));
        }

        Ok(Self {
            nullable,
            format: format.unwrap_or_else(|| DATE_DEFAULT_FORMAT.to_string()),
            range: _range,
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        let num_days_range = self.range.convert_with(|date| date.num_days_from_ce());
        let num_days_value = rng.gen_range(num_days_range);
        let date_value = SbrdDate::from_num_days_from_ce_opt(num_days_value).ok_or_else(|| {
            GenerateError::FailGenerate(format!(
                "Fail parse date from timestamp: {}",
                num_days_value
            ))
        })?;

        let evaluator = Evaluator::new(context);
        let format = evaluator.format_script(&self.format).map_err(|e| {
            GenerateError::FailEval(
                e,
                self.format.to_string(),
                context
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect::<DataValueMap<String>>(),
            )
        })?;

        Ok(DataValue::String(date_value.format(&format).to_string()))
    }
}

impl DateGenerator {
    #[inline]
    fn min_date() -> SbrdDate {
        SbrdDate::from_ymd(1900, 1, 1)
    }
    #[inline]
    fn upper_limit_date() -> SbrdDate {
        SbrdDate::from_ymd(2151, 1, 1)
    }

    fn default_range() -> ValueBound<SbrdDate> {
        ValueBound::new(
            Some(Self::min_date()),
            Some((false, Self::upper_limit_date())),
        )
    }
}
