use rand::Rng;

use crate::{DataValue, GeneratorBuilder, GeneratorType, Nullable, SbrdInt, SbrdReal, ValueBound};
use crate::generators::{Generator, get_rng};
use crate::generators::error::CompileError;

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
                    range.without_no_bound(SbrdInt::MIN as SbrdReal, SbrdInt::MAX as SbrdReal)
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

    fn generate_without_null(&self) -> DataValue {
        let mut rng = get_rng();
        let real = Rng::gen_range(&mut rng, self.range);
        DataValue::Real(real)
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
