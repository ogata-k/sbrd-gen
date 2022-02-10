use rand::Rng;

use crate::generators::error::{CompileError, GenerateError};
use crate::{DataValue, DataValueMap, GeneratorBuilder, Nullable};

pub trait Generator<R: Rng + ?Sized> {
    fn create(builder: GeneratorBuilder) -> Result<Self, CompileError>
    where
        Self: Sized;

    fn get_key(&self) -> Option<&str>;

    fn get_condition(&self) -> Option<&str>;

    fn get_nullable(&self) -> &Nullable;

    fn is_nullable(&self) -> bool {
        self.get_nullable().is_nullable()
    }

    fn is_required(&self) -> bool {
        self.get_nullable().is_required()
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
