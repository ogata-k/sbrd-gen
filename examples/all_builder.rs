//! Example for all builder's helper

use chrono::{Duration, Local, NaiveTime};
use rand::thread_rng;
use sbrd_gen::builder::{GeneratorBuilder, ParentGeneratorBuilder, ValueBound, ValueStep};
use sbrd_gen::writer::{GeneratedValueWriter, PrettyJsonWriter};
use sbrd_gen::SchemaBuilder;
use std::io::stdout;
use std::ops::Sub;
use std::path::PathBuf;

fn main() {
    let schema_builder = SchemaBuilder::new(output_list(), builder_list());
    let schema = schema_builder.build().unwrap();

    let mut rng = thread_rng();
    println!("Debug: {:?}\n", schema.generate(&mut rng).unwrap());
    println!("Display: {}\n", schema.generate(&mut rng).unwrap());

    println!("---------------------\n");

    let count = 10;
    let mut writer = PrettyJsonWriter::from_writer(stdout());
    writer
        .write_with_generate(true, &schema, &mut rng, count)
        .unwrap();
    writer.flush().unwrap();
}

fn get_schema_dir() -> PathBuf {
    let schema_dir = std::env::current_exe().unwrap();
    schema_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("examples")
        .join("schema")
}

fn output_list() -> Vec<String> {
    vec![
        "duplicate-permutation-key".to_string(),
        "format-key".to_string(),
        "dist-normal-key".to_string(),
        "eval-int-key".to_string(),
        "eval-real-key".to_string(),
        "eval-bool-key".to_string(),
        "eval-string-key".to_string(),
        "int-key".to_string(),
        "real-key".to_string(),
        "bool-key".to_string(),
        "date-time-key".to_string(),
        "date-key".to_string(),
        "time-key".to_string(),
        "always-null-key".to_string(),
        "increment-id-key".to_string(),
        "case-when-key".to_string(),
        "random-child-key".to_string(),
        "select-int-key".to_string(),
        "select-real-key".to_string(),
        "select-string-key".to_string(),
        "get-int-value-at-from-chars-key".to_string(),
        "get-int-value-at-from-values-key".to_string(),
        "get-int-value-at-from-file-key".to_string(),
        "get-real-value-at-from-chars-key".to_string(),
        "get-real-value-at-from-values-key".to_string(),
        "get-real-value-at-from-file-key".to_string(),
        "get-string-value-at-from-chars-key".to_string(),
        "get-string-value-at-from-values-key".to_string(),
        "get-string-value-at-from-file-key".to_string(),
        "get-value-index-from-chars-key".to_string(),
        "get-value-index-from-values-key".to_string(),
        "get-value-index-from-file-key".to_string(),
    ]
}

fn builder_list() -> Vec<ParentGeneratorBuilder> {
    let scheme_dir = get_schema_dir();
    let dummy_list_up_filepath = scheme_dir.join("list").join("list-up.txt");
    let dummy_num_filepath = scheme_dir.join("list").join("num.txt");

    let now = Local::now();

    vec![
        GeneratorBuilder::new_duplicate_permutation(Some(ValueBound::new(Some(3), Some((true, 5)))),
        " ", Some(vec![
                GeneratorBuilder::new_int(None).into_child(),
                GeneratorBuilder::new_real(None).into_child(),
                GeneratorBuilder::new_bool().into_child(),
                GeneratorBuilder::new_date_time(None, None).into_child(),
                GeneratorBuilder::new_date(None, None).into_child(),
                GeneratorBuilder::new_time(None, None).into_child(),
            ]),
        Some("ABC".to_string()), Some(vec![
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
                "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.".to_string(),
                "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.".to_string(),
                "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string(),
            ]), Some(dummy_list_up_filepath.clone())
        ).into_parent("duplicate-permutation-key"),
        GeneratorBuilder::new_format("Lorem ipsum: \"{duplicate-permutation-key}\"")
            .into_parent("format-key"),
        GeneratorBuilder::new_dist_normal(50.0, 10.0).into_parent("dist-normal-key"),
        GeneratorBuilder::new_eval_int("10 * 10").into_parent("eval-int-key"),
        GeneratorBuilder::new_eval_real("({dist-normal-key}-50)/10").into_parent("eval-real-key"),
        GeneratorBuilder::new_eval_bool("{dist-normal-key} < 50").into_parent("eval-bool-key"),
        GeneratorBuilder::new_eval_string("\"string: {dist-normal-key}\"").into_parent("eval-string-key"),
        GeneratorBuilder::new_int(Some(ValueBound::new(Some(1), Some((true, 100))))).into_parent("int-key"),
        GeneratorBuilder::new_real(Some((0.0 ..= 1.0).into())).into_parent("real-key"),
        GeneratorBuilder::new_bool().into_parent("bool-key"),
        GeneratorBuilder::new_date_time(Some((now.sub(Duration::days(1)).naive_local() .. now.clone().naive_local()).into()),
                                        Some("%H:%M %Y/%m/%d".to_string())).nullable().into_parent("date-time-key"),
        GeneratorBuilder::new_date(Some((now.sub(Duration::days(365)).date().naive_local() .. now.clone().date().naive_local()).into()),
                                        Some("%Y/%m/%d".to_string())).nullable().into_parent("date-key"),
        GeneratorBuilder::new_time(Some((NaiveTime::parse_from_str("00:00", "%H:%M").unwrap() ..= now.clone().time()).into()),
                                        Some("%H:%M".to_string())).nullable().into_parent("time-key"),
        GeneratorBuilder::new_always_null().into_parent("always-null-key"),
        GeneratorBuilder::new_increment_id(Some(ValueStep::new(100, Some(10)))).into_parent("increment-id-key"),
        GeneratorBuilder::new_case_when(vec![
            GeneratorBuilder::new_int(None).into_child().condition("{int-key} < 0"),
            GeneratorBuilder::new_real(None).into_child().condition("0 <= {int-key} && {int-key} < 10"),
            GeneratorBuilder::new_bool().into_child().condition("10 <= {int-key} && {int-key} < 25"),
            GeneratorBuilder::new_date_time(None, None).into_child().condition("25 <= {int-key} && {int-key} < 50"),
            GeneratorBuilder::new_date(None, None).into_child().condition("50 <= {int-key} && {int-key} < 75"),
            // default case
            GeneratorBuilder::new_time(None, None).into_child(),
        ]).into_parent("case-when-key"),
        GeneratorBuilder::new_random_child(vec![
            GeneratorBuilder::new_int(None).into_child().weight(3),
            GeneratorBuilder::new_real(None).into_child(),
            GeneratorBuilder::new_bool().into_child().weight(3),
            GeneratorBuilder::new_date_time(None, None).into_child(),
            GeneratorBuilder::new_date(None, None).into_child().weight(3),
            GeneratorBuilder::new_time(None, None).into_child(),
        ]).into_parent("random-child-key"),
        GeneratorBuilder::new_select_int(
            Some("0123456789".to_string()),
            Some(vec![10,20,30,40,50,60,70,80,90,100]),
            None
        ).into_parent("select-int-key"),
        GeneratorBuilder::new_select_real(
            Some("0123456789".to_string()),
            Some(vec![10.5,20.5,30.5,40.5,50.5,60.5,70.5,80.5,90.5,100.5]),
            None).into_parent("select-real-key"),
        GeneratorBuilder::new_select_string(
            Some("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string()),
            Some(vec![
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_string(),
                "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.".to_string(),
                "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.".to_string(),
                "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string(),
            ]),
            Some(dummy_list_up_filepath.clone())
        ).into_parent("select-string-key"),
        GeneratorBuilder::new_int(Some((0..10).into())).into_parent("int-index"),
        GeneratorBuilder::new_get_int_value_at_from_chars(
            "{int-index}",
            "0123456789"
        ).into_parent("get-int-value-at-from-chars-key"),
        GeneratorBuilder::new_get_int_value_at_from_values("{int-index}",
                                                           vec![0,10,20,30,40,50,60,70,80,90],
        ).into_parent("get-int-value-at-from-values-key"),
        GeneratorBuilder::new_get_int_value_at_from_file("{int-index}",dummy_num_filepath.clone()
        ).into_parent("get-int-value-at-from-file-key"),
        GeneratorBuilder::new_int(Some((0..10).into())).into_parent("real-index"),
        GeneratorBuilder::new_get_real_value_at_from_chars(
            "{real-index}",
            "0123456789"
        ).into_parent("get-real-value-at-from-chars-key"),
        GeneratorBuilder::new_get_real_value_at_from_values("{real-index}",
                                                            vec![0.0, 10.0,20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0],
        ).into_parent("get-real-value-at-from-values-key"),
        GeneratorBuilder::new_get_real_value_at_from_file("{real-index}",dummy_num_filepath.clone()
        ).into_parent("get-real-value-at-from-file-key"),
        GeneratorBuilder::new_int(Some((0..10).into())).into_parent("string-index"),
        GeneratorBuilder::new_get_string_value_at_from_chars(
            "{string-index}",
            "0123456789"
        ).into_parent("get-string-value-at-from-chars-key"),
        GeneratorBuilder::new_get_string_value_at_from_values(
            "{string-index}",
            vec![
                "0".to_string(),
                "10".to_string(),
                "20".to_string(),
                "30".to_string(),
                "40".to_string(),
                "50".to_string(),
                "60".to_string(),
                "70".to_string(),
                "80".to_string(),
                "90".to_string(),
            ],
        ).into_parent("get-string-value-at-from-values-key"),
        GeneratorBuilder::new_get_string_value_at_from_file("{string-index}",dummy_num_filepath.clone()
        ).into_parent("get-string-value-at-from-file-key"),
        GeneratorBuilder::new_get_value_index_from_chars(
            "0123456789"
        ).into_parent("get-value-index-from-chars-key"),
        GeneratorBuilder::new_get_value_index_from_values(
            vec![0.into(),10.into(),20.into(),30.into(),40.into(),50.into(),60.into(),70.into(),80.into(),90.into()],
        ).into_parent("get-value-index-from-values-key"),
        GeneratorBuilder::new_get_value_index_from_file(
           dummy_num_filepath.clone()
        ).into_parent("get-value-index-from-file-key"),
    ]
}
