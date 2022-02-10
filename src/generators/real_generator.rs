use rand::Rng;

use crate::generators::error::{CompileError, GenerateError};
use crate::generators::Generator;
use crate::{
    DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable, SbrdReal, ValueBound,
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct RealGenerator {
    key: Option<String>,
    condition: Option<String>,
    nullable: Nullable,
    range: ValueBound<SbrdReal>,
}
impl<R: Rng + ?Sized> Generator<R> for RealGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            key,
            range,
            condition,
            ..
        } = builder;

        if generator_type != GeneratorType::Real {
            return Err(CompileError::InvalidType(generator_type));
        }

        let default_range = Self::default_range();
        let _range = match range {
            None => default_range,
            Some(r) => r
                .try_convert_with(|s| {
                    s.parse::<SbrdReal>()
                        .map_err(|e| CompileError::InvalidValue(e.to_string()))
                })
                .map(|range| {
                    // 範囲指定がないと[0, 1)で生成されてしまうため上限下限を設定する
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
