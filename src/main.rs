use rand::thread_rng;

use sbrd_gen::error::{ErrorKind, SbrdGenError};
use sbrd_gen::*;

fn main() {
    let parent_builder = GeneratorBuilder::new_dist_normal(0.0, 1.0)
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
