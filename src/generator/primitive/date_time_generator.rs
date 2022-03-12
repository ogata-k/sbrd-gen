use crate::builder::{GeneratorBuilder, Nullable, ValueBound};
use crate::error::{BuildError, GenerateError};
use crate::eval::Evaluator;
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap, SbrdDate, SbrdDateTime, DATE_TIME_DEFAULT_FORMAT};
use crate::GeneratorType;

/// The generator with generate [`SbrdDateTime`] value as [`DataValue::String`] with the format
///
/// See [`format::strftime` module] for more information on `format` option.
/// The default for `format` and the format when parsing is [`DATE_TIME_DEFAULT_FORMAT`].
///
/// [`SbrdDateTime`]: ../../value/type.SbrdDateTime.html
/// [`DataValue::String`]: ../../value/enum.DataValue.html#variant.String
/// [`format::strftime` module]: https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers
/// [`DATE_TIME_DEFAULT_FORMAT`]: ../../value/constant.DATE_TIME_DEFAULT_FORMAT.html
#[derive(Debug, PartialEq, Clone)]
pub struct DateTimeGenerator {
    nullable: Nullable,
    format: String,
    range: ValueBound<SbrdDateTime>,
}

impl<R: Randomizer + ?Sized> Generator<R> for DateTimeGenerator {
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

        if generator_type != GeneratorType::DateTime {
            return Err(BuildError::InvalidType(generator_type));
        }

        let default_range = Self::default_range();
        let _range = match range {
            None => default_range,
            Some(r) => r
                .try_convert_with(|s| {
                    SbrdDateTime::parse_from_str(&s.to_parse_string(), DATE_TIME_DEFAULT_FORMAT)
                        .map_err(|e| {
                            BuildError::FailParseValue(
                                s.to_parse_string(),
                                "DateTime".to_string(),
                                e.to_string(),
                            )
                        })
                })
                .map(|range| {
                    // 生成可能な範囲で生成できるように範囲指定を実装
                    range.without_no_bound_from_other(default_range)
                })?,
        };
        if _range.is_empty() {
            return Err(BuildError::RangeEmpty(_range.convert_into()));
        }

        Ok(Self {
            nullable,
            format: format.unwrap_or_else(|| DATE_TIME_DEFAULT_FORMAT.to_string()),
            range: _range,
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        let timestamp_range = self.range.convert_with(|date_time| date_time.timestamp());
        let timestamp_value = rng.gen_range(timestamp_range);
        let date_time_value =
            SbrdDateTime::from_timestamp_opt(timestamp_value, 0).ok_or_else(|| {
                GenerateError::FailGenerate(format!(
                    "Fail parse date time from timestamp: {}",
                    timestamp_value
                ))
            })?;

        let evaluator = Evaluator::new(&self.format, context);
        let format = evaluator.format_script().map_err(|e| {
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

        Ok(DataValue::String(
            date_time_value.format(&format).to_string(),
        ))
    }
}

impl DateTimeGenerator {
    #[inline]
    fn min_date_time() -> SbrdDateTime {
        SbrdDate::from_ymd(1900, 1, 1).and_hms(0, 0, 0)
    }
    #[inline]
    fn max_date_time() -> SbrdDateTime {
        SbrdDate::from_ymd(2151, 1, 1).and_hms(0, 0, 0)
    }

    fn default_range() -> ValueBound<SbrdDateTime> {
        ValueBound::new(
            Some(Self::min_date_time()),
            Some((false, Self::max_date_time())),
        )
    }
}
