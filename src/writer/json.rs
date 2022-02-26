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
use serde_json::ser::{CompactFormatter, PrettyFormatter};
use serde_json::Serializer as JsonSerializer;
use std::borrow::BorrowMut;
use std::io;
use std::marker::PhantomData;

pub struct CompactJsonWriter<W: io::Write> {
    json_writer: JsonWriter<W, CompactFormatter>,
}

impl<W: io::Write> GeneratedValueWriter<W> for CompactJsonWriter<W> {
    fn from_writer(writer: W) -> Self {
        Self {
            json_writer: JsonWriter::from_writer(writer),
        }
    }

    fn into_inner(self) -> W {
        self.json_writer.into_inner()
    }

    fn flush(&mut self) -> SchemaResult<()> {
        self.json_writer.flush()
    }

    fn write_after_all_generated<R: Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()> {
        self.json_writer
            .write_after_all_generated(use_key_header, schema, rng, count)
    }

    fn write_with_generate<R: Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()> {
        self.json_writer
            .write_with_generate(use_key_header, schema, rng, count)
    }
}

pub struct PrettyJsonWriter<'a, W: io::Write> {
    json_writer: JsonWriter<W, PrettyFormatter<'a>>,
}

impl<'a, W: io::Write> GeneratedValueWriter<W> for PrettyJsonWriter<'a, W> {
    fn from_writer(writer: W) -> Self {
        Self {
            json_writer: JsonWriter::from_writer(writer),
        }
    }

    fn into_inner(self) -> W {
        self.json_writer.into_inner()
    }

    fn flush(&mut self) -> SchemaResult<()> {
        self.json_writer.flush()
    }

    fn write_after_all_generated<R: Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()> {
        self.json_writer
            .write_after_all_generated(use_key_header, schema, rng, count)
    }

    fn write_with_generate<R: Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()> {
        self.json_writer
            .write_with_generate(use_key_header, schema, rng, count)
    }
}

trait BuildJsonFormatter: serde_json::ser::Formatter {
    fn build_formatter() -> Self;
}

impl BuildJsonFormatter for CompactFormatter {
    fn build_formatter() -> Self {
        CompactFormatter
    }
}

impl<'a> BuildJsonFormatter for PrettyFormatter<'a> {
    fn build_formatter() -> Self {
        PrettyFormatter::new()
    }
}

struct JsonWriter<W: io::Write, F: BuildJsonFormatter> {
    writer: W,
    formatter: PhantomData<F>,
}

impl<W: io::Write, F: BuildJsonFormatter> JsonWriter<W, F> {
    fn from_writer(writer: W) -> Self {
        Self {
            writer,
            formatter: PhantomData,
        }
    }

    fn build_serializer(&mut self) -> JsonSerializer<&mut W, F> {
        JsonSerializer::with_formatter(&mut self.writer, F::build_formatter())
    }

    fn flush(&mut self) -> SchemaResult<()> {
        self.writer
            .flush()
            .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))
    }

    fn into_inner(self) -> W {
        self.writer
    }

    fn write_after_all_generated<R: Randomizer + ?Sized>(
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

    fn write_with_generate<R: Randomizer + ?Sized>(
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
