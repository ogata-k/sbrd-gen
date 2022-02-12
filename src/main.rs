use rand::thread_rng;

use sbrd_gen::error::{ErrorKind, SbrdGenError};
use sbrd_gen::*;

fn main() {
    let with_key_builder = GeneratorBuilder::new_case_when([
        (
            "250 <= {dummy_int}",
            GeneratorBuilder::new_date_time(
                Some(ValueBound::new(
                    Some(SbrdDate::from_ymd(2021, 12, 25).and_hms(0, 0, 0)),
                    Some((false, SbrdDate::from_ymd(2021, 12, 26).and_hms(0, 0, 0))),
                )),
                None,
            ),
        )
            .into(),
        (
            "-150 <= {dummy_int} && {dummy_int} < 250",
            GeneratorBuilder::new_date(
                Some(ValueBound::new(
                    Some(SbrdDate::from_ymd(2021, 12, 25)),
                    None,
                )),
                None,
            ),
        )
            .into(),
        (
            "-400 <= {dummy_int} && {dummy_int} < -150",
            GeneratorBuilder::new_time(
                Some(ValueBound::new(
                    None,
                    Some((true, SbrdTime::from_hms(23, 59, 59))),
                )),
                None,
            ),
        )
            .into(),
        GeneratorBuilder::new_real(Some((0.0..=1.0).into())).into(),
    ])
    .with_key("KeyA");
    let yaml_string = serde_yaml::to_string(&with_key_builder)
        .map_err(|e| e.into_sbrd_gen_error(ErrorKind::SerializeError))
        .unwrap();
    println!("[builder]\n{}", &yaml_string);

    let deserialized: WithKeyBuilder = serde_yaml::from_str(&yaml_string)
        .map_err(|e| e.into_sbrd_gen_error(ErrorKind::ParseError))
        .unwrap();
    // println!("[builder]\n{:?}", deserialized);

    assert_eq!(deserialized, with_key_builder);

    let (key, builder) = with_key_builder.split();
    let generator = builder
        .build()
        .map_err(|e| e.into_sbrd_gen_error(ErrorKind::BuildError))
        .unwrap();

    let mut rng = thread_rng();
    println!("[generate for \"{}\"]", key);
    for _ in 0..10 {
        let mut value_map = DataValueMap::new();
        value_map.insert(
            "dummy_int".to_string(),
            GeneratorBuilder::new_int(Some((-500..=500).into()))
                .build()
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::BuildError))
                .unwrap()
                .generate(&mut rng, &value_map)
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::GenerateError))
                .unwrap(),
        );
        value_map.insert(
            "dummy_real".to_string(),
            GeneratorBuilder::new_real(None)
                .build()
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::BuildError))
                .unwrap()
                .generate(&mut rng, &value_map)
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::GenerateError))
                .unwrap(),
        );
        value_map.insert(
            "dummy_bool".to_string(),
            GeneratorBuilder::new_bool()
                .build()
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::BuildError))
                .unwrap()
                .generate(&mut rng, &value_map)
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::GenerateError))
                .unwrap(),
        );
        value_map.insert(
            "dummy_date_time".to_string(),
            GeneratorBuilder::new_date_time(None, Option::<String>::None)
                .build()
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::BuildError))
                .unwrap()
                .generate(&mut rng, &value_map)
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::GenerateError))
                .unwrap(),
        );
        value_map.insert(
            "dummy_date".to_string(),
            GeneratorBuilder::new_date(None, Option::<String>::None)
                .build()
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::BuildError))
                .unwrap()
                .generate(&mut rng, &value_map)
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::GenerateError))
                .unwrap(),
        );
        value_map.insert(
            "dummy_time".to_string(),
            GeneratorBuilder::new_time(None, Option::<String>::None)
                .build()
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::BuildError))
                .unwrap()
                .generate(&mut rng, &value_map)
                .map_err(|e| e.into_sbrd_gen_error(ErrorKind::GenerateError))
                .unwrap(),
        );

        let generate_value = generator
            .generate(&mut rng, &value_map)
            .map_err(|e| e.into_sbrd_gen_error(ErrorKind::GenerateError))
            .unwrap();
        println!("{}", generate_value);
    }
}
