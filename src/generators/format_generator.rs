use rand::Rng;

use crate::generators::error::{CompileError, GenerateError};
use crate::generators::Generator;
use crate::{replace_values, DataValue, DataValueMap, GeneratorBuilder, GeneratorType, Nullable};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct FormatGenerator {
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
            format,
            ..
        } = builder;

        if generator_type != GeneratorType::Format {
            return Err(CompileError::InvalidType(generator_type));
        }

        match format {
            None => Err(CompileError::NotExistValueOfKey("format".to_string())),
            Some(_format) => Ok(Self {
                nullable,
                format: _format,
            }),
        }
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
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
