use crate::builder::{GeneratorBuilder, Nullable, ValueBound};
use crate::error::{BuildError, GenerateError};
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap, SbrdInt};
use crate::GeneratorType;

/// The generator with generate [`DataValue::Int`] value with range of generated value.
///
/// [`DataValue::Int`]: ../../value/enum.DataValue.html#variant.Int
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IntGenerator {
    nullable: Nullable,
    range: ValueBound<SbrdInt>,
}

impl<R: Randomizer + ?Sized> Generator<R> for IntGenerator {
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

        if generator_type != GeneratorType::Int {
            return Err(BuildError::InvalidType(generator_type));
        }

        let default_range = Self::default_range();
        let _range = match range {
            None => default_range,
            Some(r) => r.try_convert_with(|s| {
                s.to_parse_string().parse::<SbrdInt>().map_err(|e| {
                    BuildError::FailParseValue(
                        s.to_parse_string(),
                        "Int".to_string(),
                        e.to_string(),
                    )
                })
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
        _value_map: &DataValueMap<&str>,
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
