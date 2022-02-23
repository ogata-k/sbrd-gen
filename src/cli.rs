#![allow(deprecated)]

use clap::{AppSettings, ArgEnum, Parser};
use std::path::PathBuf;

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
#[deny(missing_docs)]
pub struct SbrdGenApp {
    /// Scheme for generate value
    ///
    /// You must specify `keys` and `builders` as sequence in the scheme.
    /// To learn more about the schema, see: https://github.com/ogata-k/sbrd-gen/README.md#About%20Scheme
    scheme_file_path: PathBuf,

    /// Type of Parser for scheme
    #[clap(short = 'p', long = "parser", arg_enum, default_value_t = ParserType::Yaml)]
    parser: ParserType,

    /// Type of Output for this generator
    #[clap(short = 't', long = "type", arg_enum, default_value_t = OutputType::Json)]
    output_type: OutputType,

    /// Count of generate values
    #[clap(short = 'n', long = "num", default_value = "10")]
    count: u32,

    /// Flag for generate without key's header
    #[clap(long = "no-header")]
    no_header: bool,

    /// Flag for only check scheme
    #[clap(long = "dry-run")]
    dry_run: bool,
}

impl SbrdGenApp {
    pub fn run(self) -> ! {
        todo!()
    }
}
