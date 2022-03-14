use crate::builder::{GeneratorBuilder, ValueBound};
use crate::error::{BuildError, GenerateError};
use crate::eval::Evaluator;
use crate::generator::{GeneratorBase, Randomizer};
use crate::value::{DataValue, DataValueMap, SbrdTime, TIME_DEFAULT_FORMAT};
use crate::GeneratorType;
use chrono::Duration;
use std::ops::AddAssign;

/// The generator with generate [`SbrdTime`] value as [`DataValue::String`] with the format
///
/// See [`format::strftime` module] for more information on `format` option.
/// The default for `format` and the format when parsing is [`TIME_DEFAULT_FORMAT`].
///
/// [`SbrdTime`]: ../../value/type.SbrdTime.html
/// [`DataValue::String`]: ../../value/enum.DataValue.html#variant.String
/// [`format::strftime` module]: https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers
/// [`TIME_DEFAULT_FORMAT`]: ../../value/constant.TIME_DEFAULT_FORMAT.html
#[derive(Debug, PartialEq, Clone)]
pub struct TimeGenerator {
    nullable: bool,
    format: String,
    range: ValueBound<SbrdTime>,
}

impl<R: Randomizer + ?Sized> GeneratorBase<R> for TimeGenerator {
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

        if generator_type != GeneratorType::Time {
            return Err(BuildError::InvalidType(generator_type));
        }

        let default_range = Self::default_range();
        let _range = match range {
            None => default_range,
            Some(r) => r
                .try_convert_with(|s| {
                    SbrdTime::parse_from_str(&s.to_parse_string(), TIME_DEFAULT_FORMAT).map_err(
                        |e| {
                            BuildError::FailParseValue(
                                s.to_parse_string(),
                                "Time".to_string(),
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
            format: format.unwrap_or_else(|| TIME_DEFAULT_FORMAT.to_string()),
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
        let upper_bound = self
            .range
            .get_end()
            .expect("Exist upper bound is not exist");
        let lower_bound = self
            .range
            .get_start()
            .expect("Exist lower bound is not exist");
        let since_duration_seconds = upper_bound.signed_duration_since(lower_bound).num_seconds();
        let diff_seconds = rng.gen_range(ValueBound::new(
            Some(0),
            Some((self.range.is_include_end(), since_duration_seconds)),
        ));
        let mut time_value = lower_bound;
        time_value.add_assign(Duration::seconds(diff_seconds));

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

        Ok(DataValue::String(time_value.format(&format).to_string()))
    }
}

impl TimeGenerator {
    #[inline]
    fn min_time() -> SbrdTime {
        SbrdTime::from_hms(0, 0, 0)
    }
    #[inline]
    fn max_time() -> SbrdTime {
        SbrdTime::from_hms(23, 59, 59)
    }

    fn default_range() -> ValueBound<SbrdTime> {
        ValueBound::new(Some(Self::min_time()), Some((true, Self::max_time())))
    }
}
