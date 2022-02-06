use rand::{Rng, thread_rng};

use crate::{DataValue, GeneratorBuilder, Nullable};
use crate::generators::error::{CompileError, GenerateError};

pub fn get_rng() -> impl Rng {
    thread_rng()
}

pub trait Generator {
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

    fn generate(&self) -> Result<DataValue, GenerateError> {
        if self.is_required() {
            self.generate_without_null()
        } else {
            if Rng::gen_bool(&mut get_rng(), 0.1) {
                return Ok(DataValue::Null);
            }

            self.generate_without_null()
        }
    }

    fn generate_without_null(&self) -> Result<DataValue, GenerateError> ;
}
