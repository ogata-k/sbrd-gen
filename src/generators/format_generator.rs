use rand::Rng;

use crate::generators::error::{CompileError, GenerateError};
use crate::generators::Generator;
use crate::{replace_values, DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct FormatGenerator {
    key: Option<String>,
    condition: Option<String>,
    nullable: Nullable,
    format: String,
}
impl<R: Rng + ?Sized> Generator<R> for FormatGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            key,
            condition,
            format,
            ..
        } = builder;

        if generator_type != GeneratorType::Format {
            return Err(CompileError::InvalidType(generator_type));
        }

        match format {
            None => Err(CompileError::NotExistValueOfKey("format".to_string())),
            Some(mut _format) => Ok(Self {
                key,
                condition,
                nullable,
                format: _format,
            }),
        }
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
        _rng: &mut R,
        value_map: &DataValueMap<String>,
    ) -> Result<DataValue, GenerateError> {
        let format = replace_values(&self.format, value_map);
        Ok(DataValue::String(format))
    }
}
