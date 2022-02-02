use std::collections::BTreeMap;
use std::path::PathBuf;

use sbrd_gen::*;

fn main() {
    let generator_a = GeneratorBuilder {
        key: Some("KeyA".to_string()),
        condition: Some("$KeyA$ == 10".to_string()),
        generator_type: GeneratorType::Select,
        nullable: Nullable::new_as_nullable(),
        bound: Some(ValueBound{
            start: Some("0.0".to_string()),
            end: Some("19.0".to_string()),
            include_end: false
        }),
        path: Some(PathBuf::from("hoge.csv")),
        select_values: Some(vec![
            DataValue::String("KeyA".to_string()),
            DataValue::Bool(false),
            DataValue::Int(32),
        ]),
        format: Some("$KeyA$KeyA".to_string()),
        dist_parameters: Some(BTreeMap::from([(
            "hogheoge".to_string(),
            DataValue::Int(10),
        )])),
        children: Some(vec![
            GeneratorBuilder {
                key: Some("KeyA-1".to_string()),
                condition: Some("$KeyA == 10".to_string()),
                generator_type: GeneratorType::DistIntUniform,
                nullable: Nullable::new_as_required(),
                bound:  Some(ValueBound{
                    start: None,
                    end: Some("19.0".to_string()),
                    include_end: false
                }),
                path: Some(PathBuf::from("hoge.csv")),
                select_values: Some(vec![
                    DataValue::String("KeyA".to_string()),
                    DataValue::Bool(false),
                    DataValue::Int(32),
                ]),
                format: Some("$KeyA$KeyA".to_string()),
                dist_parameters: Some(BTreeMap::from([(
                    "hogheoge".to_string(),
                    DataValue::Int(10),
                )])),
                children: None,
            },
            GeneratorBuilder {
                key: None,
                condition: None,
                generator_type: GeneratorType::Int,
                nullable: Nullable::new_as_required(),
                bound: Some(ValueBound{
                    start: Some("0.0".to_string()),
                    end: None,
                    include_end: false
                }),
                path: None,
                select_values: None,
                format: None,
                dist_parameters: None,
                children: None,
            },
        ]),
    };

    let dummy = Scheme::new(
        vec!["KeyA".to_string(), "KeyB".to_string()],
        vec![generator_a],
    );
    println!("{}", serde_yaml::to_string(&dummy).unwrap());
}
