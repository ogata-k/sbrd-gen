//! Example for all builder from yaml

use std::fs::File;
use std::io::stdout;
use std::path::PathBuf;
use rand::thread_rng;
use sbrd_gen::file::set_schema_file_path;
use sbrd_gen::parser::YamlParser;
use sbrd_gen::parser::SchemaParser;
use sbrd_gen::writer::{GeneratedValueWriter, PrettyJsonWriter};

fn main()
{
    let schema_dir = get_schema_dir();
    let schema_file_path = schema_dir.join("all.yaml");

    // set load current filepath
    set_schema_file_path(schema_file_path.clone());

    let file = File::open(schema_file_path.as_path()).unwrap();

    let schema_builder = YamlParser::parse_from_reader(file).unwrap();
    let schema = schema_builder.build().unwrap();

    let mut rng = thread_rng();
    println!("Debug: {:?}\n", schema.generate(&mut rng).unwrap());
    println!("Display: {}\n", schema.generate(&mut rng).unwrap());

    println!("---------------------\n");

    let count = 10;
    let mut writer = PrettyJsonWriter::from_writer(stdout());
    writer.write_with_generate(true, &schema, &mut rng, count).unwrap();
    writer.flush().unwrap();
}

fn get_schema_dir()->PathBuf
{
    let schema_dir = std::env::current_exe().unwrap();
    schema_dir
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .join("examples")
        .join("schema")
}