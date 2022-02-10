use rand::Rng;

use crate::generators::error::{CompileError, GenerateError};
use crate::generators::Generator;
use crate::{
    DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable, SbrdDate, SbrdDateTime,
    ValueBound, DATE_TIME_DEFAULT_FORMAT,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct DateTimeGenerator {
    key: Option<String>,
    condition: Option<String>,
    nullable: Nullable,
    format: String,
    range: ValueBound<SbrdDateTime>,
}
impl<R: Rng + ?Sized> Generator<R> for DateTimeGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            key,
            range,
            format,
            condition,
            ..
        } = builder;

        if generator_type != GeneratorType::DateTime {
            return Err(CompileError::InvalidType(generator_type));
        }

        let default_range = Self::default_range();
        let _range = match range {
            None => default_range,
            Some(r) => r
                .try_convert_with(|s| {
                    SbrdDateTime::parse_from_str(&s, DATE_TIME_DEFAULT_FORMAT)
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
            key,
            condition,
            nullable,
            format: format.unwrap_or_else(|| DATE_TIME_DEFAULT_FORMAT.to_string()),
            range: _range,
        })
    }

    fn get_key(&self) -> Option<&str> {
        self.key.as_ref().map(|s| s.as_ref())
    }

    fn get_condition(&self) -> Option<&str> {
        self.condition.as_ref().map(|s| s.as_ref())
    }

    fn get_nullable(&self) -> &Nullable {
        &self.nullable
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        _value_map: &DataValueMap<String>,
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
        Ok(DataValue::String(
            date_time_value.format(&self.format).to_string(),
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
