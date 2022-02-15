use crate::builder::GeneratorBuilder;
use crate::generator::error::{CompileError, GenerateError};
use crate::value::{DataValue, DataValueMap};
use rand::Rng;

pub trait Randomizer: Rng {}
impl<R: Rng> Randomizer for R {}

pub trait Generator<R: Randomizer + ?Sized> {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized;

    fn is_nullable(&self) -> bool;

    fn is_required(&self) -> bool {
        !self.is_nullable()
    }

    fn generate(&self, rng: &mut R, value_map: &DataValueMap) -> Result<DataValue, GenerateError> {
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
        value_map: &DataValueMap,
    ) -> Result<DataValue, GenerateError>;
}
