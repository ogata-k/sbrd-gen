use crate::error::{IntoSbrdError, SchemaErrorKind, SchemaResult};
use crate::generator::Randomizer;
use crate::value::DataValue;
use crate::writer::GeneratedValueWriterBase;
use crate::Schema;
use csv::QuoteStyle;
use serde::Serialize;
use std::io;

/// A writer that outputs as Comma-Separated Values for the key and the generated value
pub struct CsvWriter<W: io::Write> {
    xsv_writer: XsvWriter<W>,
}

impl<W: io::Write> GeneratedValueWriterBase<W> for CsvWriter<W> {
    fn from_writer(writer: W) -> Self {
        let xsv_writer = XsvWriter::from_writer(writer, b',');

        Self { xsv_writer }
    }

    fn into_inner(self) -> W {
        self.xsv_writer.into_inner()
    }

    fn flush(&mut self) -> SchemaResult<()> {
        self.xsv_writer.flush()
    }

    fn write_after_all_generated<R: Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()> {
        self.xsv_writer
            .write_after_all_generated(use_key_header, schema, rng, count)
    }

    fn write_with_generate<R: Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()> {
        self.xsv_writer
            .write_with_generate(use_key_header, schema, rng, count)
    }
}

/// A writer that outputs as Tab-Separated Values for the key and the generated value
pub struct TsvWriter<W: io::Write> {
    xsv_writer: XsvWriter<W>,
}

impl<W: io::Write> GeneratedValueWriterBase<W> for TsvWriter<W> {
    fn from_writer(writer: W) -> Self {
        let xsv_writer = XsvWriter::from_writer(writer, b'\t');

        Self { xsv_writer }
    }

    fn into_inner(self) -> W {
        self.xsv_writer.into_inner()
    }

    fn flush(&mut self) -> SchemaResult<()> {
        self.xsv_writer.flush()
    }

    fn write_after_all_generated<R: Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()> {
        self.xsv_writer
            .write_after_all_generated(use_key_header, schema, rng, count)
    }

    fn write_with_generate<R: Randomizer + ?Sized>(
        &mut self,
        use_key_header: bool,
        schema: &Schema<R>,
        rng: &mut R,
        count: u64,
    ) -> SchemaResult<()> {
        self.xsv_writer
            .write_with_generate(use_key_header, schema, rng, count)
    }
}

/// A writer that outputs as XSV for the key and the generated value
struct XsvWriter<W: io::Write> {
    writer: W,
    delimiter: u8,
}

impl<W: io::Write> XsvWriter<W> {
    fn build_xsv_writer(&self) -> csv::Writer<Vec<u8>> {
        csv::WriterBuilder::new()
            .delimiter(self.delimiter)
            .has_headers(false)
            .flexible(false)
            .quote_style(QuoteStyle::Necessary)
            .from_writer(vec![])
    }

    fn from_writer(writer: W, delimiter: u8) -> Self {
        Self { writer, delimiter }
    }

    fn flush(&mut self) -> SchemaResult<()> {
        self.writer
            .flush()
            .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))
    }

    fn serialize<S: Serialize>(&mut self, record: S) -> SchemaResult<()> {
        let mut xsv_writer = self.build_xsv_writer();
        xsv_writer
            .serialize(record)
            .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
        xsv_writer
            .flush()
            .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;
        let format_input = xsv_writer
            .into_inner()
            .map_err(|e| e.into_sbrd_gen_error(SchemaErrorKind::OutputError))?;

        self.writer
            .write_all(format_input.as_slice())
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
        let mut values_list: Vec<Vec<DataValue>> = Vec::new();
        for _ in 1..=count {
            let generated = schema.generate(rng)?;
            let values = generated.into_values()?;

            values_list.push(values);
        }

        if use_key_header {
            self.serialize(schema.get_keys())?;
        }
        for values in values_list {
            self.serialize(values)?;
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
        if use_key_header {
            self.serialize(schema.get_keys())?;
        }

        for _ in 1..=count {
            let generated = schema.generate(rng)?;
            let values = generated.into_values()?;

            self.serialize(values)?;
        }

        self.flush()?;
        Ok(())
    }
}
