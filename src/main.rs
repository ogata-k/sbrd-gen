use rand::thread_rng;

use sbrd_gen::error::{ErrorKind, SbrdGenError};
use sbrd_gen::*;

fn main() {
    let parent_builder = GeneratorBuilder::new_duplicate_permutation_with_children(
        None,
        ", ",
        vec![
            GeneratorBuilder::new_duplicate_permutation_with_select_list(
                Some(ValueBound::new(Some(6), Some((true, 15)))),
                "",
                Some("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-~/\"!\\&%".to_string()),
                None, None
            ).into_child(),
            GeneratorBuilder::new_duplicate_permutation_with_select_list(
                Some(ValueBound::new(Some(3), Some((true, 5)))),
                " ",
                None,
                Some(vec![
                    "I am Lorem Ipsum.".to_string(),
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
                    "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.".to_string(),
                    "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.".to_string(),
                    "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string(),
                ]), None
            ).into_child(),
            GeneratorBuilder::new_select_int(
                None,
                Some(vec![
                    1,2,3,4,5,6,7,8,9,0,
                ]),
                None
            ).into_child(),
            GeneratorBuilder::new_date_time(
                Some(ValueBound::new(
                    Some(SbrdDate::from_ymd(2021, 12, 25).and_hms(0, 0, 0)),
                    Some((false, SbrdDate::from_ymd(2021, 12, 26).and_hms(0, 0, 0))),
                )),
                None,
            )
            .into_child()
            .weight(2),
            GeneratorBuilder::new_date(
                Some(ValueBound::new(
                    Some(SbrdDate::from_ymd(2021, 12, 25)),
                    None,
                )),
                None,
            )
            .into_child()
            // is same to specify weight
            .weight(1),
            GeneratorBuilder::new_time(
                Some(ValueBound::new(
                    None,
                    Some((true, SbrdTime::from_hms(23, 59, 59))),
                )),
                None,
            )
            .into_child()
            // no choice
            .weight(0),
            GeneratorBuilder::new_real(Some((0.0..=1.0).into()))
                .into_child()
                .weight(3),
        ],
    )
    .into_parent("KeyA");
    let yaml_string = serde_yaml::to_string(&parent_builder)
        .map_err(|e| e.into_sbrd_gen_error(ErrorKind::SerializeError))
        .unwrap();
    println!("[builder]\n{}", &yaml_string);

    let deserialized: ParentGeneratorBuilder = serde_yaml::from_str(&yaml_string)
        .map_err(|e| e.into_sbrd_gen_error(ErrorKind::ParseError))
        .unwrap();
    // println!("[builder]\n{:?}", deserialized);

    assert_eq!(deserialized, parent_builder);

    let (key, builder) = parent_builder.split_key();
    let generator = builder
        .build()
        .map_err(|e| e.into_sbrd_gen_error(ErrorKind::BuildError))
        .unwrap();

    let mut rng = thread_rng();
    println!("[generate for \"{}\"]", key);
    for index in 1..=10 {
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
        println!("{} : {}", index, generate_value);
    }
}
