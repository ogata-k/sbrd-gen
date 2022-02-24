use crate::error::SchemaResult;
use crate::generator::Randomizer;
use crate::Schema;
use serde::ser::{Error, SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use std::sync::Mutex;

pub trait GeneratedValueWriter<W: std::io::Write> {
    fn from_writer(writer: W) -> Self;
    fn into_inner(self) -> W;
    fn flush(&mut self) -> SchemaResult<()>;
    fn write_after_all_generated<R: 'static + Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()>;
    fn write_with_generate<R: 'static + Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()>;
}

pub const DUMMY_KEYS_NAME: &str = "keys";
pub const DUMMY_VALUES_NAME: &str = "values";

pub struct GeneratedDisplayValues<K: Serialize, V: Serialize> {
    key_values: Vec<(K, V)>,
}

impl<K: Serialize, V: Serialize> GeneratedDisplayValues<K, V> {
    pub(in crate::writer) fn new(key_values: Vec<(K, V)>) -> Self {
        Self { key_values }
    }
}

impl<K: Serialize, V: Serialize> Serialize for GeneratedDisplayValues<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map_state = serializer.serialize_map(Some(self.key_values.len()))?;
        for (k, v) in self.key_values.iter() {
            map_state.serialize_entry(k, v)?;
        }
        map_state.end()
    }
}

pub struct SerializeWithGenerate<'a, R: 'static + Randomizer + ?Sized> {
    schema: &'a Schema<R>,
    rng: Mutex<&'a mut R>,
    count: &'a u64,
}

impl<'a, R: 'static + Randomizer + ?Sized> SerializeWithGenerate<'a, R> {
    pub(in crate::writer) fn new(schema: &'a Schema<R>, rng: &'a mut R, count: &'a u64) -> Self {
        Self {
            schema,
            rng: Mutex::new(rng),
            count,
        }
    }
}

impl<'a, R: 'static + Randomizer + ?Sized> Serialize for SerializeWithGenerate<'a, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq_state = serializer.serialize_seq(Some(*self.count as usize))?;

        for _ in 0..*self.count {
            let generated = {
                let mut rng = self.rng.try_lock().map_err(S::Error::custom)?;
                self.schema.generate(*rng).map_err(S::Error::custom)?
            };

            let values = generated.into_values_with_key().map_err(S::Error::custom)?;

            let json_map = GeneratedDisplayValues::new(values);
            seq_state.serialize_element(&json_map)?;
        }

        seq_state.end()
    }
}
