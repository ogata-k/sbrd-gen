use rand::thread_rng;

use sbrd_gen::error::{ErrorKind, SbrdGenError};
use sbrd_gen::*;

fn main() {
    let int_generator = GeneratorBuilder::new_int(None).with_key("KeyA");

    let select_sting_generator = GeneratorBuilder::new_select_string_with_values([
        "KeyA".to_string(),
        false.to_string(),
        32_u8.to_string(),
        SbrdDate::from_ymd(2015, 9, 5)
            .and_hms(23, 56, 4)
            .to_string(),
        SbrdDate::from_ymd(2015, 9, 5).to_string(),
        SbrdTime::from_hms(23, 56, 4).to_string(),
    ])
    .with_key("KeyB");
    let duplicate_permutation_generator = GeneratorBuilder::new_duplicate_permutation_with_chars(
        None,
        "",
        "abcdefghijklmn!\"#$%&'\\",
    )
    .nullable()
    .with_key("KeyC");
    // other case get error or null(when nullable)
    let case_when_generator = GeneratorBuilder::new_case_when([
        (
            "{KeyA} < 0",
            GeneratorBuilder::new_date_time(
                Some(ValueBound::new(
                    Some(SbrdDate::from_ymd(2021, 12, 25).and_hms(0, 0, 0)),
                    Some((false, SbrdDate::from_ymd(2021, 12, 26).and_hms(0, 0, 0))),
                )),
                None,
            )
            .nullable(),
        )
            .into(),
        (
            "0 <= {KeyA} && {KeyA} < 100",
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
            "100 <= {KeyA} && {KeyA} < 200",
            GeneratorBuilder::new_time(
                Some(ValueBound::new(
                    None,
                    Some((true, SbrdTime::from_hms(23, 59, 59))),
                )),
                None,
            ),
        )
            .into(),
    ])
    .with_key("KeyD");
    let format_generator = GeneratorBuilder::new_format("{KeyC} {KeyD}").with_key("KeyE");

    let dummy = Scheme::new(
        vec!["KeyA".to_string(), "KeyB".to_string(), "KeyE".to_string()],
        vec![
            int_generator,
            select_sting_generator,
            duplicate_permutation_generator,
            case_when_generator,
            format_generator,
        ],
    );
    let yaml_string = serde_yaml::to_string(&dummy)
        .map_err(|e| e.into_sbrd_gen_error(ErrorKind::SerializeError))
        .unwrap();
    println!("[schema]\n{}", &yaml_string);

    let deserialized: Scheme = serde_yaml::from_str(&yaml_string)
        .map_err(|e| e.into_sbrd_gen_error(ErrorKind::ParseError))
        .unwrap();
    // println!("[schema]\n{:?}", deserialized);

    assert_eq!(deserialized, dummy);

    println!("\n---------------------------------------------------------------------------\n");

    let with_key_builder = GeneratorBuilder::new_case_when([
        (
            "50 <= {dummy_int}",
            GeneratorBuilder::new_date_time(
                Some(ValueBound::new(
                    Some(SbrdDate::from_ymd(2021, 12, 25).and_hms(0, 0, 0)),
                    Some((false, SbrdDate::from_ymd(2021, 12, 26).and_hms(0, 0, 0))),
                )),
                None,
            )
            .nullable(),
        )
            .into(),
        (
            "10 <= {dummy_int} && {dummy_int} < 50",
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
            "0 <= {dummy_int} && {dummy_int} < 10",
            GeneratorBuilder::new_time(
                Some(ValueBound::new(
                    None,
                    Some((true, SbrdTime::from_hms(23, 59, 59))),
                )),
                None,
            ),
        )
            .into(),
        GeneratorBuilder::new_always_null().into(),
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
        .map_err(|e| e.into_sbrd_gen_error(ErrorKind::GenerateError))
        .unwrap();

    let mut rng = thread_rng();
    println!("[generate for \"{}\"]", key);
    for _ in 0..10 {
        let mut value_map = DataValueMap::new();
        value_map.insert(
            "dummy_int".to_string(),
            GeneratorBuilder::new_int(None)
                .build()
                .unwrap()
                .generate(&mut rng, &value_map)
                .unwrap(),
        );
        value_map.insert(
            "dummy_real".to_string(),
            GeneratorBuilder::new_real(None)
                .build()
                .unwrap()
                .generate(&mut rng, &value_map)
                .unwrap(),
        );
        value_map.insert(
            "dummy_bool".to_string(),
            GeneratorBuilder::new_bool()
                .build()
                .unwrap()
                .generate(&mut rng, &value_map)
                .unwrap(),
        );
        value_map.insert(
            "dummy_date_time".to_string(),
            GeneratorBuilder::new_date_time(None, Option::<String>::None)
                .build()
                .unwrap()
                .generate(&mut rng, &value_map)
                .unwrap(),
        );
        value_map.insert(
            "dummy_date".to_string(),
            GeneratorBuilder::new_date(None, Option::<String>::None)
                .build()
                .unwrap()
                .generate(&mut rng, &value_map)
                .unwrap(),
        );
        value_map.insert(
            "dummy_time".to_string(),
            GeneratorBuilder::new_time(None, Option::<String>::None)
                .build()
                .unwrap()
                .generate(&mut rng, &value_map)
                .unwrap(),
        );

        println!("{:?}", generator.generate(&mut rng, &value_map));
    }
}
