use crate::error::{IntoSbrdError, SchemeErrorKind, SchemeResult};
use crate::generator::Randomizer;
use crate::value::DataValue;
use crate::writer::writer_base::{
    GeneratedDisplayValues, SerializeWithGenerate, DUMMY_KEYS_NAME, DUMMY_VALUES_NAME,
};
use crate::writer::GeneratedValueWriter;
use crate::Scheme;
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

    fn flush(&mut self) -> SchemeResult<()> {
        self.writer
            .flush()
            .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))
    }

    fn write_after_all_generated<R: 'static + Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        scheme: &Scheme<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemeResult<()> {
        let mut values_list: Vec<GeneratedDisplayValues<String, DataValue>> = Vec::new();
        for _ in 1..=count {
            let generated = scheme.generate(rng)?;
            let values = generated.into_values_with_key()?;

            let value_map = GeneratedDisplayValues::new(values);
            values_list.push(value_map);
        }

        let mut serializer = self.build_serializer();
        if use_key_header {
            let mut map_state = serializer
                .borrow_mut()
                .serialize_map(Some(2))
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
            SerializeMap::serialize_entry(&mut map_state, DUMMY_KEYS_NAME, scheme.get_keys())
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
            SerializeMap::serialize_entry(
                &mut map_state,
                DUMMY_VALUES_NAME,
                values_list.as_slice(),
            )
            .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
            SerializeMap::end(map_state)
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
        } else {
            let mut seq_state = serializer
                .borrow_mut()
                .serialize_seq(Some(values_list.len()))
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
            for values in values_list.iter() {
                SerializeSeq::serialize_element(&mut seq_state, values)
                    .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
            }
            SerializeSeq::end(seq_state)
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
        }

        self.flush()?;
        Ok(())
    }

    fn write_with_generate<R: 'static + Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        scheme: &Scheme<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemeResult<()> {
        let mut serializer = self.build_serializer();
        if use_key_header {
            let mut map_state = serializer
                .borrow_mut()
                .serialize_map(Some(2))
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
            SerializeMap::serialize_entry(&mut map_state, DUMMY_KEYS_NAME, scheme.get_keys())
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
            SerializeMap::serialize_entry(
                &mut map_state,
                DUMMY_VALUES_NAME,
                &SerializeWithGenerate::new(scheme, rng, &count),
            )
            .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
            SerializeMap::end(map_state)
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
        } else {
            let mut seq_state = serializer
                .borrow_mut()
                .serialize_seq(Some(count as usize))
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
            for _ in 1..=count {
                let generated = scheme.generate(rng)?;
                let values = generated.into_values_with_key()?;

                let value_map = GeneratedDisplayValues::new(values);
                SerializeSeq::serialize_element(&mut seq_state, &value_map)
                    .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
            }
            SerializeSeq::end(seq_state)
                .map_err(|e| e.into_sbrd_gen_error(SchemeErrorKind::OutputError))?;
        }

        self.flush()?;
        Ok(())
    }
}
