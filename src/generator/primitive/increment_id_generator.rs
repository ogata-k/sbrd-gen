use crate::builder::{GeneratorBuilder, Nullable};
use crate::error::{BuildError, GenerateError};
use crate::generator::{Generator, Randomizer};
use crate::value::{DataValue, DataValueMap, SbrdInt};
use crate::GeneratorType;
use std::cell::Cell;

const INITIAL_ID: SbrdInt = 1;
const DEFAULT_STEP: SbrdInt = 1;

/// The generator with generate integer value with the initial value and the step value.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IncrementIdGenerator {
    nullable: Nullable,
    current_id: Cell<SbrdInt>,
    step: SbrdInt,
}

impl<R: Randomizer + ?Sized> Generator<R> for IncrementIdGenerator {
    fn create(builder: GeneratorBuilder) -> Result<Self, BuildError>
    where
        Self: Sized,
    {
        let GeneratorBuilder {
            generator_type,
            nullable,
            increment,
            ..
        } = builder;

        if generator_type != GeneratorType::IncrementId {
            return Err(BuildError::InvalidType(generator_type));
        }

        let (initial_id, step): (SbrdInt, SbrdInt) = match increment {
            None => (INITIAL_ID, DEFAULT_STEP),
            Some(s) => {
                let _initial_id: SbrdInt = s
                    .get_initial()
                    .to_parse_string()
                    .parse::<SbrdInt>()
                    .map_err(|e| {
                        BuildError::FailParseValue(
                            s.get_initial().to_parse_string(),
                            "Int".to_string(),
                            e.to_string(),
                        )
                    })?;

                let step_result = s.get_step().as_ref().map(|v| {
                    v.to_parse_string().parse::<SbrdInt>().map_err(|e| {
                        BuildError::FailParseValue(
                            v.to_parse_string(),
                            "Int".to_string(),
                            e.to_string(),
                        )
                    })
                });

                match step_result {
                    None => (_initial_id, DEFAULT_STEP),
                    Some(_step_result) => (_initial_id, _step_result?),
                }
            }
        };

        Ok(Self {
            nullable,
            current_id: Cell::new(initial_id),
            step,
        })
    }

    fn is_nullable(&self) -> bool {
        self.nullable.is_nullable()
    }

    fn generate_without_null(
        &self,
        _rng: &mut R,
        _value_map: &DataValueMap<&str>,
    ) -> Result<DataValue, GenerateError> {
        let id = self.current_id.get();
        self.current_id.replace(id + self.step);

        Ok(DataValue::Int(id))
    }
}
