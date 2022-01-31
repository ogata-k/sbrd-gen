use rand::Rng;

use crate::DataValue;

pub trait Generator {
    fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> DataValue;
}
