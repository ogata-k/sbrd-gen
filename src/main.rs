use rand::thread_rng;
use sbrd_gen::builder::GeneratorBuilder;
use sbrd_gen::writer::{CsvWriter, GeneratedValueWriter};
use sbrd_gen::SchemeBuilder;
use std::io::stdout;

fn main() {
    let keys = vec!["KeyA".to_string(), "KeyB".to_string(), "KeyC".to_string()];
    let builders = vec![
        GeneratorBuilder::new_dist_normal(0.0, 1.0).into_parent("KeyA"),
        GeneratorBuilder::new_int(Some((-500..=500).into())).into_parent("dummy_int"),
        GeneratorBuilder::new_real(None).into_parent("dummy_real"),
        GeneratorBuilder::new_bool().into_parent("dummy_bool"),
        GeneratorBuilder::new_date_time(None, None).into_parent("dummy_date_time"),
        GeneratorBuilder::new_date(None, None).into_parent("dummy_date"),
        GeneratorBuilder::new_time(None, None).into_parent("dummy_time"),
        GeneratorBuilder::new_format("{dummy_date_time}, {dummy_date} {dummy_time}")
            .into_parent("KeyB"),
        GeneratorBuilder::new_eval_bool("\"{dummy_date_time}\" == \"{dummy_date} {dummy_time}\"")
            .into_parent("KeyC"),
    ];
    let scheme = SchemeBuilder::new(keys, builders).build().unwrap();

    let mut rng = thread_rng();
    let mut writer = CsvWriter::from_writer(stdout());

    writer
        .write_with_generate(true, &scheme, &mut rng, 10)
        .unwrap();
}
