use crate::query::DataSource;
use crate::query::Dimension;
use crate::query::model::Aggregation;
use crate::query::Granularity;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "queryType", rename = "topN")]
pub struct TopN {
    // todo: data_source would result in weird error message
    pub data_source: DataSource,
    pub dimension: Dimension,
    pub threshold: usize,
    pub metric: String,
    pub aggregations: Vec<Aggregation>,
    pub intervals: Vec<String>,
    pub granularity: Granularity,
    pub context: std::collections::HashMap<String, String>,
}