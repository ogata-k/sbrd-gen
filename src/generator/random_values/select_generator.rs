use crate::builder::GeneratorBuilder;
use crate::error::{BuildError, GenerateError};
use crate::generator::{GeneratorBase, Randomizer, ValueGeneratorBase};
use crate::value::{DataValue, DataValueMap, SbrdInt, SbrdReal, SbrdString};
use crate::GeneratorType;
use rand::seq::SliceRandom;
use std::str::FromStr;

/// The generator with generate value as the type T from value's list as the type
pub struct SelectGenerator<T> {
    nullable: bool,
    selectable_values: Vec<T>,
}

impl<R: Randomizer + ?Sized, T: ForSelectGeneratorType> ValueGeneratorBase<R, T>
    for SelectGenerator<T>
{
    fn parse(input: &str) -> Result<T, BuildError> {
        T::parse(input)
    }
}

impl<R: Randomizer + ?Sized, T: ForSelectGeneratorType> GeneratorBase<R> for SelectGenerator<T> {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            chars,
            values,
            filepath,
            ..
        } = builder;

        if generator_type != T::get_generator_type() {
            return Err(BuildError::InvalidType(generator_type));
        }

        let selectable_values =
            <Self as ValueGeneratorBase<R, T>>::build_selectable(chars, values, filepath)?;

        Ok(Self {
            nullable,
            selectable_values,
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        _context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        self.selectable_values
            .choose(rng)
            .map(|v| v.to_data_value())
            .ok_or_else(|| GenerateError::FailGenerate("Fail Select Value".to_string()))
    }
}

/// Helper traits for generators that the generate value
pub trait ForSelectGeneratorType {
    /// The type of the generator
    fn get_generator_type() -> GeneratorType;

    /// Function of parser the input value
    fn parse(s: &str) -> Result<Self, BuildError>
    where
        Self: Sized;

    /// Function of converter for a generated value
    fn to_data_value(&self) -> DataValue;
}

impl ForSelectGeneratorType for SbrdInt {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::SelectInt
    }

    fn parse(s: &str) -> Result<SbrdInt, BuildError> {
        SbrdInt::from_str(s).map_err(|e| {
            BuildError::FailParseValue(s.to_string(), "Int".to_string(), e.to_string())
        })
    }

    fn to_data_value(&self) -> DataValue {
        DataValue::Int(*self)
    }
}

impl ForSelectGeneratorType for SbrdReal {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::SelectReal
    }

    fn parse(s: &str) -> Result<SbrdReal, BuildError> {
        SbrdReal::from_str(s).map_err(|e| {
            BuildError::FailParseValue(s.to_string(), "Real".to_string(), e.to_string())
        })
    }

    fn to_data_value(&self) -> DataValue {
        DataValue::Real(*self)
    }
}

impl ForSelectGeneratorType for SbrdString {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::SelectString
    }

    fn parse(s: &str) -> Result<String, BuildError> {
        Ok(s.to_string())
    }

    fn to_data_value(&self) -> DataValue {
        DataValue::String(self.to_string())
    }
}
