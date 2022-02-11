use chrono::Duration;
use rand::Rng;
use std::ops::AddAssign;

use crate::generators::error::{CompileError, GenerateError};
use crate::generators::Generator;
use crate::{
    DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable, SbrdTime, ValueBound,
    TIME_DEFAULT_FORMAT,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct TimeGenerator {
    nullable: Nullable,
    format: String,
    range: ValueBound<SbrdTime>,
}

impl<R: Rng + ?Sized> Generator<R> for TimeGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
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
            return Err(CompileError::InvalidType(generator_type));
        }

        let default_range = Self::default_range();
        let _range = match range {
            None => default_range,
            Some(r) => r
                .try_convert_with(|s| {
                    SbrdTime::parse_from_str(&s.to_parse_string(), TIME_DEFAULT_FORMAT)
                        .map_err(|e| CompileError::InvalidValue(e.to_string()))
                })
                .map(|range| {
                    // 生成可能な範囲で生成できるように範囲指定を実装
                    range.without_no_bound_from_other(default_range)
                })?,
        };
        if _range.is_empty() {
            return Err(CompileError::RangeEmpty(
                _range.convert_with(|b| b.to_string()),
            ));
        }

        Ok(Self {
            nullable,
            format: format.unwrap_or_else(|| TIME_DEFAULT_FORMAT.to_string()),
            range: _range,
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        _value_map: &DataValueMap<String>,
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

        Ok(DataValue::String(
            time_value.format(&self.format).to_string(),
        ))
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
