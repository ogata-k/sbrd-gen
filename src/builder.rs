use std::collections::btree_map::BTreeMap;
use std::path::PathBuf;

use serde::Serialize;

use crate::bound::ValueBound;
use crate::generator_type::GeneratorType;
use crate::value::DataValue;

// TODO fieldのpubを外す
#[derive(Serialize, Debug, PartialEq, Clone)]
pub struct GeneratorBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "type")]
    pub generator_type: GeneratorType,
    pub nullable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bound: Option<ValueBound<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_values: Option<Vec<DataValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dist_parameters: Option<BTreeMap<String, DataValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<GeneratorBuilder>>,
}
