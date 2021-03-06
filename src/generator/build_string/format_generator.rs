use crate::builder::GeneratorBuilder;
use crate::error::{BuildError, GenerateError};
use crate::eval::Evaluator;
use crate::generator::{GeneratorBase, Randomizer};
use crate::value::{DataValue, DataValueMap};
use crate::GeneratorType;

/// The generator with [`DataValue::String`] formatted by specified `format` with evaluating by [`Evaluator`]
///
/// [`DataValue::String`]: ../../value/enum.DataValue.html#variant.String
/// [`Evaluator`]: ../../eval/struct.Evaluator.html
#[derive(Debug, PartialEq, Clone)]
pub struct FormatGenerator {
    nullable: bool,
    format: String,
}

impl<R: Randomizer + ?Sized> GeneratorBase<R> for FormatGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
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
            return Err(BuildError::InvalidType(generator_type));
        }

        match format {
            None => Err(BuildError::NotExistValueOf("format".to_string())),
            Some(_format) => Ok(Self {
                nullable,
                format: _format,
            }),
        }
    }

    fn is_nullable(&self) -> bool {
        self.nullable
    }

    fn generate_without_null(
        &self,
        _rng: &mut R,
        context: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        Evaluator::new(context)
            .format_script(&self.format)
            .map(DataValue::String)
            .map_err(|e| {
                GenerateError::FailEval(
                    e,
                    self.format.to_string(),
                    context
                        .clone()
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v))
                        .collect::<DataValueMap<String>>(),
                )
            })
    }
}
