use crate::builder::{GeneratorBuilder, Nullable};
use crate::error::{BuildError, GenerateError};
use crate::eval::Evaluator;
use crate::generator::{Generator, Randomizer, SingleValueOptionGenerator};
use crate::value::{DataValue, DataValueMap, SbrdInt, SbrdReal, SbrdString};
use crate::GeneratorType;
use std::str::FromStr;

/// The generator that gets a T value from the values with the value evaluated by `script` as the index of 0-index.
pub struct GetValueAtGenerator<T> {
    nullable: Nullable,
    script: String,
    selectable_values: Vec<T>,
}

impl<R: Randomizer + ?Sized, T: ForGetValueAtGeneratorType> SingleValueOptionGenerator<R, T>
    for GetValueAtGenerator<T>
{
    fn parse(input: &str) -> Result<T, BuildError> {
        T::parse(input)
    }
}

impl<R: Randomizer + ?Sized, T: ForGetValueAtGeneratorType> Generator<R>
    for GetValueAtGenerator<T>
{
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
            script,
            ..
        } = builder;

        if generator_type != T::get_generator_type() {
            return Err(BuildError::InvalidType(generator_type));
        }

        let selectable_values =
            <Self as SingleValueOptionGenerator<R, T>>::build_selectable(chars, values, filepath)?;

        match script {
            None => Err(BuildError::NotExistValueOf("script".to_string())),
            Some(script) => Ok(Self {
                nullable,
                script,
                selectable_values,
            }),
        }
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        _rng: &mut R,
        value_map: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        let evaluator = Evaluator::new(&self.script, value_map);
        let index: usize = evaluator.eval_int().map(|v| v as usize).map_err(|e| {
            GenerateError::FailEval(
                e,
                self.script.clone(),
                value_map
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.clone()))
                    .collect::<DataValueMap<String>>(),
            )
        })?;

        match self.selectable_values.get(index) {
            None => Err(GenerateError::FailGenerate(format!(
                "Not found value at index {}",
                index
            ))),
            Some(v) => Ok(v.to_data_value()),
        }
    }
}

/// Helper traits for generators that the generate value
pub trait ForGetValueAtGeneratorType {
    /// The type of the generator
    fn get_generator_type() -> GeneratorType;

    /// Function of parser the input value
    fn parse(s: &str) -> Result<Self, BuildError>
    where
        Self: Sized;

    /// Function of converter for a generated value
    fn to_data_value(&self) -> DataValue;
}

impl ForGetValueAtGeneratorType for SbrdInt {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::GetIntValueAt
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

impl ForGetValueAtGeneratorType for SbrdReal {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::GetRealValueAt
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

impl ForGetValueAtGeneratorType for SbrdString {
    fn get_generator_type() -> GeneratorType {
        GeneratorType::GetStringValueAt
    }

    fn parse(s: &str) -> Result<String, BuildError> {
        Ok(s.to_string())
    }

    fn to_data_value(&self) -> DataValue {
        DataValue::String(self.to_string())
    }
}
