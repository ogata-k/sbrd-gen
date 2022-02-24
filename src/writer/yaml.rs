use crate::error::{IntoSbrdError, SchemaErrorKind, SchemaResult};
use crate::generator::Randomizer;
use crate::value::DataValue;
use crate::writer::writer_base::{
    GeneratedDisplayValues, SerializeWithGenerate, DUMMY_KEYS_NAME, DUMMY_VALUES_NAME,
};
use crate::writer::GeneratedValueWriter;
use crate::Schema;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::Serializer;
use serde_yaml::Serializer as YamlSerializer;
use std::borrow::BorrowMut;
use std::io;

pub struct YamlWriter<W: io::Write> {
    writer: W,
}

impl<W: io::Write> YamlWriter<W> {
    fn build_serializer(&mut self) -> YamlSerializer<&mut W> {
        YamlSerializer::new(&mut self.writer)
    }
}

impl<W: io::Write> GeneratedValueWriter<W> for YamlWriter<W> {
    fn from_writer(writer: W) -> Self {
        Self { writer }
    }

    fn into_inner(self) -> W {
        self.writer
    }

    fn flush(&mut self) -> SchemaResult<()> {
        self.writer
            .flush()
            .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))
    }

    fn write_after_all_generated<R: 'static + Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()> {
        let mut values_list: Vec<GeneratedDisplayValues<String, DataValue>> = Vec::new();
        for _ in 1..=count {
            let generated = schema.generate(rng)?;
            let values = generated.into_values_with_key()?;

            let value_map = GeneratedDisplayValues::new(values);
            values_list.push(value_map);
        }

        let mut serializer = self.build_serializer();
        if use_key_header {
            let mut map_state = serializer
                .borrow_mut()
                .serialize_map(Some(2))
                .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
            SerializeMap::serialize_entry(&mut map_state, DUMMY_KEYS_NAME, schema.get_keys())
                .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
            SerializeMap::serialize_entry(
                &mut map_state,
                DUMMY_VALUES_NAME,
                values_list.as_slice(),
            )
            .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
            SerializeMap::end(map_state)
                .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
        } else {
            let mut seq_state = serializer
                .borrow_mut()
                .serialize_seq(Some(values_list.len()))
                .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
            for values in values_list.iter() {
                SerializeSeq::serialize_element(&mut seq_state, values)
                    .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
            }
            SerializeSeq::end(seq_state)
                .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
        }

        self.flush()?;
        Ok(())
    }

    fn write_with_generate<R: 'static + Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()> {
        let mut serializer = self.build_serializer();
        if use_key_header {
            let mut map_state = serializer
                .borrow_mut()
                .serialize_map(Some(2))
                .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
            SerializeMap::serialize_entry(&mut map_state, DUMMY_KEYS_NAME, schema.get_keys())
                .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
            SerializeMap::serialize_entry(
                &mut map_state,
                DUMMY_VALUES_NAME,
                &SerializeWithGenerate::new(schema, rng, &count),
            )
            .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
            SerializeMap::end(map_state)
                .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
        } else {
            let mut seq_state = serializer
                .borrow_mut()
                .serialize_seq(Some(count as usize))
                .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
            for _ in 1..=count {
                let generated = schema.generate(rng)?;
                let values = generated.into_values_with_key()?;

                let value_map = GeneratedDisplayValues::new(values);
                SerializeSeq::serialize_element(&mut seq_state, &value_map)
                    .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
            }
            SerializeSeq::end(seq_state)
                .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
        }

        self.flush()?;
        Ok(())
    }
}
