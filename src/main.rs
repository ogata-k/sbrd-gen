use chrono::{NaiveDate, NaiveTime};

use sbrd_gen::*;
use sbrd_gen::error::{ErrorKind, SbrdGenError};

fn main() {
    let int_generator = GeneratorBuilder::new_int(None).with_key("KeyA");

    let select_sting_generator = GeneratorBuilder::new_select_string_with_values([
        "KeyA".to_string(),
        false.to_string(),
        32_u8.to_string(),
        NaiveDate::from_ymd(2015, 9, 5)
            .and_hms(23, 56, 4)
            .to_string(),
        NaiveDate::from_ymd(2015, 9, 5).to_string(),
        NaiveTime::from_hms(23, 56, 4).to_string(),
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
    let when_generator = GeneratorBuilder::new_when([
        (
            "{KeyA} < 0",
            GeneratorBuilder::new_date_time(Some(ValueBound::new(
                Some(NaiveDate::from_ymd(2021, 12, 25).and_hms(0, 0, 0)),
                Some((false, NaiveDate::from_ymd(2021, 12, 26).and_hms(0, 0, 0))),
            )))
            .nullable(),
        ),
        (
            "0 <= {KeyA} && {KeyA} < 100",
            GeneratorBuilder::new_date(Some(ValueBound::new(
                Some(NaiveDate::from_ymd(2021, 12, 25)),
                None,
            ))),
        ),
        (
            "100 <= {KeyA} && {KeyA} < 200",
            GeneratorBuilder::new_time(Some(ValueBound::new(
                None,
                Some((true, NaiveTime::from_hms(23, 59, 59))),
            ))),
        ),
    ])
    .with_key("KeyD");
    let format_generator = GeneratorBuilder::new_format("{KeyC} {KeyD}").with_key("KeyE");

    let dummy = Scheme::new(
        vec!["KeyA".to_string(), "KeyB".to_string(), "KeyE".to_string()],
        vec![
            int_generator,
            select_sting_generator,
            duplicate_permutation_generator,
            when_generator,
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

    let builder = GeneratorBuilder::new_int(Some((-100..100).into()))
        .with_key("KeyA")
        .nullable();
    let yaml_string = serde_yaml::to_string(&builder)
        .map_err(|e| e.into_sbrd_gen_error(ErrorKind::SerializeError))
        .unwrap();
    println!("[builder]\n{}", &yaml_string);

    let deserialized: GeneratorBuilder = serde_yaml::from_str(&yaml_string)
        .map_err(|e| e.into_sbrd_gen_error(ErrorKind::ParseError))
        .unwrap();
    // println!("[builder]\n{:?}", deserialized);

    assert_eq!(deserialized, builder);

    let int_generator = builder.build().unwrap();
    println!("[generate]\n{:?}", int_generator.generate());
}
