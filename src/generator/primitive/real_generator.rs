use crate::builder::{GeneratorBuilder, Nullable, ValueBound};
use crate::error::{BuildError, GenerateError};
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap, SbrdReal};
use crate::GeneratorType;

#[derive(Debug, PartialEq, Clone)]
pub struct RealGenerator {
    nullable: Nullable,
    range: ValueBound<SbrdReal>,
}

impl<R: Randomizer + ?Sized> Generator<R> for RealGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            range,
            ..
        } = builder;

        if generator_type != GeneratorType::Real {
            return Err(BuildError::InvalidType(generator_type));
        }

        let default_range = Self::default_range();
        let _range = match range {
            None => default_range,
            Some(r) => r
                .try_convert_with(|s| {
                    s.to_parse_string().parse::<SbrdReal>().map_err(|e| {
                        BuildError::FailParseValue(
                            s.to_parse_string(),
                            "Real".to_string(),
                            e.to_string(),
                        )
                    })
                })
                .map(|range| {
                    // 範囲指定がないと[0, 1)で生成されてしまうため上限下限を設定する
                    range.without_no_bound_from_other(default_range)
                })?,
        };
        if _range.is_empty() {
            return Err(BuildError::RangeEmpty(_range.convert_into()));
        }

        Ok(Self {
            nullable,
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
        let real = rng.gen_range(self.range);

        Ok(DataValue::Real(real))
    }
}

impl RealGenerator {
    fn default_range() -> ValueBound<SbrdReal> {
        ValueBound::new(
            Some(i16::MIN as SbrdReal),
            Some((true, i16::MAX as SbrdReal)),
        )
    }
}
