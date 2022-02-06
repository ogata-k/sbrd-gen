use rand::Rng;

use crate::{DataValue, GeneratorBuilder, GeneratorType, Nullable, SbrdInt, ValueBound};
use crate::generators::{Generator, get_rng};
use crate::generators::error::CompileError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct IntGenerator {
    key: Option<String>,
    condition: Option<String>,
    nullable: Nullable,
    range: ValueBound<SbrdInt>,
}

impl Generator for IntGenerator {
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

        if generator_type != GeneratorType::Int {
            return Err(CompileError::InvalidType(generator_type));
        }

        let default_range = Self::default_range();
        let _range = match range {
            None => default_range,
            Some(r) => r.try_convert_with(|s| {
                s.parse::<SbrdInt>()
                    .map_err(|e| CompileError::InvalidValue(e.to_string()))
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
        let v: SbrdInt = get_rng().gen_range(self.range);

        DataValue::Int(v)
    }
}

impl IntGenerator {
    fn default_range() -> ValueBound<SbrdInt> {
        ValueBound::new(Some(i16::MIN as SbrdInt), Some((true, i16::MAX as SbrdInt)))
    }
}
