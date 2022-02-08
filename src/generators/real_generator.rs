use rand::Rng;

use crate::{
    DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable, SbrdReal, ValueBound,
};
use crate::generators::{Generator, get_rng};
use crate::generators::error::{CompileError, GenerateError};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct RealGenerator {
    key: Option<String>,
    condition: Option<String>,
    nullable: Nullable,
    range: ValueBound<SbrdReal>,
}
impl Generator for RealGenerator {
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
                    range.without_no_bound(i16::MIN as SbrdReal, i16::MAX as SbrdReal)
                })?,
        };

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
        _value_map: &DataValueMap<String>,
    ) -> Result<DataValue, GenerateError> {
        if self.range.is_empty() {
            return Err(GenerateError::RangeEmpty(
                self.range.convert_with(|r| r.to_string()),
            ));
        }

        let mut rng = get_rng();
        let real = Rng::gen_range(&mut rng, self.range);
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
