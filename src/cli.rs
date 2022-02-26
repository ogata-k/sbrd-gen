#![allow(deprecated)]

use clap::{AppSettings, ArgEnum, Parser};
use rand::prelude::ThreadRng;
use rand::thread_rng;
use sbrd_gen::error::{BuildError, SchemaResult};
use sbrd_gen::file::set_schema_file_path;
use sbrd_gen::generator::Randomizer;
use sbrd_gen::parser::{JsonParser, SchemaParser, YamlParser};
use sbrd_gen::writer::{CsvWriter, GeneratedValueWriter, PrettyJsonWriter, TsvWriter, YamlWriter};
use sbrd_gen::{Schema, SchemaBuilder};
use std::fs::File;
use std::io;
use std::io::{stdout, BufWriter, Stdout};
use std::path::PathBuf;
use std::process::exit;

#[derive(ArgEnum, Debug, Eq, PartialEq, Copy, Clone)]
#[clap(rename_all = "kebab-case")]
pub enum ParserType {
    Yaml,
    Json,
}

#[derive(ArgEnum, Debug, Eq, PartialEq, Copy, Clone)]
#[clap(rename_all = "kebab-case")]
pub enum OutputType {
    Yaml,
    Json,
    Csv,
    Tsv,
}

#[derive(Parser, Debug, PartialEq, Eq, Clone)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(global_setting(AppSettings::NextLineHelp))]
pub struct SbrdGenApp {
    /// Schema for generate value
    ///
    /// You must specify `keys` and `builders` as sequence in the schema.
    /// To learn more about the schema, see: https://github.com/ogata-k/sbrd-gen/blob/v0.1.x/README.md#about-schema
    schema_file_path: PathBuf,

    /// Type of Parser for schema
    #[clap(short = 'p', long = "parser", arg_enum, default_value_t = ParserType::Yaml)]
    parser_type: ParserType,

    /// Type of Output for this generator
    #[clap(short = 't', long = "type", arg_enum, default_value_t = OutputType::Json)]
    output_type: OutputType,

    /// Count of generate values
    #[clap(short = 'n', long = "num", default_value = "10")]
    count: u64,

    /// Flag for generate without key's header
    #[clap(long = "no-header")]
    no_header: bool,

    /// Flag for only check schema
    #[clap(long = "dry-run")]
    dry_run: bool,
}

impl SbrdGenApp {
    pub fn run(self) -> ! {
        // set load current filepath
        set_schema_file_path(self.schema_file_path.as_path());

        let file = File::open(self.schema_file_path.as_path()).unwrap_or_else(|e| {
            eprintln!(
                "{}",
                BuildError::FileError(e, self.schema_file_path.clone())
            );
            exit(exitcode::IOERR);
        });

        let schema_builder: SchemaBuilder = match self.parser_type {
            ParserType::Yaml => YamlParser::parse_from_reader(file),
            ParserType::Json => JsonParser::parse_from_reader(file),
        }
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(exitcode::IOERR);
        });

        let schema = schema_builder.build().unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(exitcode::IOERR);
        });

        if self.dry_run {
            println!("Parse Succeed");
            exit(exitcode::OK);
        }

        type Rng = ThreadRng;
        let mut rng = thread_rng();

        type Output = BufWriter<Stdout>;
        let output = BufWriter::new(stdout());
        let output_result: SchemaResult<()> = match self.output_type {
            OutputType::Yaml => {
                self.write_all_data::<Output, YamlWriter<Output>, Rng>(output, &schema, &mut rng)
            }
            OutputType::Json => {
                // use human readable json writer
                self.write_all_data::<Output, PrettyJsonWriter<Output>, Rng>(
                    output, &schema, &mut rng,
                )
            }
            OutputType::Csv => {
                self.write_all_data::<Output, CsvWriter<Output>, Rng>(output, &schema, &mut rng)
            }
            OutputType::Tsv => {
                self.write_all_data::<Output, TsvWriter<Output>, Rng>(output, &schema, &mut rng)
            }
        };

        output_result.unwrap_or_else(|e| {
            eprintln!("{}", e);
            exit(exitcode::SOFTWARE);
        });

        exit(exitcode::OK)
    }

    fn write_all_data<O, Writer, R>(
        &self,
        output: O,
        schema: &Schema<R>,
        rng: &mut R,
    ) -> SchemaResult<()>
    where
        O: io::Write,
        Writer: GeneratedValueWriter<O>,
        R: 'static + Randomizer + ?Sized,
    {
        let mut writer = Writer::from_writer(output);
        writer
            .write_with_generate(!self.no_header, schema, rng, self.count)
            .map_err(|e| match writer.flush() {
                Ok(()) => e,
                Err(flush_error) => flush_error,
            })
    }
}
