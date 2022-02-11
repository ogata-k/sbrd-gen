use rand::Rng;

use crate::generators::error::{CompileError, GenerateError};
use crate::{DataValue, DataValueMap, GeneratorBuilder};

pub trait Generator<R: Rng + ?Sized> {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized;

    fn is_nullable(&self) -> bool;

    fn is_required(&self) -> bool {
        !self.is_nullable()
    }

    fn generate(
        &self,
        rng: &mut R,
        value_map: &DataValueMap<String>,
    ) -> Result<DataValue, GenerateError> {
        if self.is_required() {
            self.generate_without_null(rng, value_map)
        } else {
            if rng.gen_bool(0.1) {
                return Ok(DataValue::Null);
            }

            self.generate_without_null(rng, value_map)
        }
    }

    fn generate_without_null(
        &self,
        rng: &mut R,
        value_map: &DataValueMap<String>,
    ) -> Result<DataValue, GenerateError>;
}
