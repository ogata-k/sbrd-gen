use rand::Rng;

use crate::generators::error::{CompileError, GenerateError};
use crate::generators::Generator;
use crate::{
    DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable, SbrdInt, ValueBound,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct IntGenerator {
    nullable: Nullable,
    range: ValueBound<SbrdInt>,
}

impl<R: Rng + ?Sized> Generator<R> for IntGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            range,
            ..
        } = builder;

        if generator_type != GeneratorType::Int {
            return Err(CompileError::InvalidType(generator_type));
        }

        let default_range = Self::default_range();
        let _range = match range {
            None => default_range,
            Some(r) => r.try_convert_with(|s| {
                s.to_parse_string()
                    .parse::<SbrdInt>()
                    .map_err(|e| CompileError::InvalidValue(e.to_string()))
            })?,
        };
        if _range.is_empty() {
            return Err(CompileError::RangeEmpty(_range.convert_into()));
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
        let v: SbrdInt = rng.gen_range(self.range);

        Ok(DataValue::Int(v))
    }
}

impl IntGenerator {
    fn default_range() -> ValueBound<SbrdInt> {
        ValueBound::new(Some(i16::MIN as SbrdInt), Some((true, i16::MAX as SbrdInt)))
    }
}
