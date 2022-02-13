use crate::generators::error::{CompileError, GenerateError};
use crate::generators::{Generator, Randomizer};
use crate::{
    DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable, SbrdDate, ValueBound,
    DATE_DEFAULT_FORMAT,
};
use chrono::Datelike;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct DateGenerator {
    nullable: Nullable,
    format: String,
    range: ValueBound<SbrdDate>,
}

impl<R: Randomizer + ?Sized> Generator<R> for DateGenerator {
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

        if generator_type != GeneratorType::Date {
            return Err(CompileError::InvalidType(generator_type));
        }

        let default_range = Self::default_range();
        let _range = match range {
            None => default_range,
            Some(r) => r
                .try_convert_with(|s| {
                    SbrdDate::parse_from_str(&s.to_parse_string(), DATE_DEFAULT_FORMAT)
                        .map_err(|e| CompileError::InvalidValue(e.to_string()))
                })
                .map(|range| {
                    // 生成可能な範囲で生成できるように範囲指定を実装
                    range.without_no_bound_from_other(default_range)
                })?,
        };
        if _range.is_empty() {
            return Err(CompileError::RangeEmpty(_range.convert_into()));
        }

        Ok(Self {
            nullable,
            format: format.unwrap_or_else(|| DATE_DEFAULT_FORMAT.to_string()),
            range: _range,
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        _value_map: &DataValueMap,
    ) -> Result<DataValue, GenerateError> {
        let num_days_range = self.range.convert_with(|date| date.num_days_from_ce());
        let num_days_value = rng.gen_range(num_days_range);
        let date_value = SbrdDate::from_num_days_from_ce_opt(num_days_value).ok_or_else(|| {
            GenerateError::FailGenerate(format!(
                "Fail parse date from timestamp: {}",
                num_days_value
            ))
        })?;

        Ok(DataValue::String(
            date_value.format(&self.format).to_string(),
        ))
    }
}

impl DateGenerator {
    #[inline]
    fn min_date() -> SbrdDate {
        // 1900/1/1
        SbrdDate::from_num_days_from_ce(693596)
    }
    #[inline]
    fn max_date() -> SbrdDate {
        // 2151/1/1
        SbrdDate::from_num_days_from_ce(785272)
    }

    fn default_range() -> ValueBound<SbrdDate> {
        ValueBound::new(Some(Self::min_date()), Some((false, Self::max_date())))
    }
}
