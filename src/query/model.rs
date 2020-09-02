use super::SortingOrder;
use crate::query::DataSource;
use crate::query::Dimension;
use crate::query::Filter;
use crate::query::Granularity;
use crate::query::Ordering;
use serde::{Deserialize, Serialize};

// }
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "queryType")]
#[serde(rename_all = "camelCase")]
pub enum Query {
    #[serde(rename_all = "camelCase")]
    TopN {
        // todo: data_source would result in weird error message
        data_source: DataSource,
        dimension: Dimension,
        threshold: usize,
        metric: String,
        aggregations: Vec<Aggregation>,
        intervals: Vec<String>,
        granularity: Granularity,
    },
    #[serde(rename_all = "camelCase")]
    Scan {
        data_source: DataSource,
        intervals: Vec<String>,
        result_format: ResultFormat,
        filter: Option<Filter>,
        columns: Vec<String>,
        batch_size: usize,
        limit: Option<usize>,
        ordering: Option<Ordering>,
        context: std::collections::HashMap<String, String>,
    },
    #[serde(rename_all = "camelCase")]
    GroupBy {
        data_source: DataSource,
        dimensions: Vec<Dimension>,
        limit_spec: Option<LimitSpec>,
        having: Option<HavingSpec>,
        granularity: Granularity,
        filter: Option<Filter>,
        aggregations: Vec<Aggregation>,
        post_aggregations: Vec<PostAggregation>,
        intervals: Vec<String>,
        subtotal_spec: Vec<Vec<String>>,
        context: std::collections::HashMap<String, String>,
    },
    #[serde(rename_all = "camelCase")]
    Search {
        data_source: DataSource,
        granularity: Granularity,
        filter: Option<Filter>,
        limit: usize,
        intervals: Vec<String>,
        search_dimensions: Vec<String>,
        query: SearchQuerySpec,
        sort: Option<SortingOrder>,
        context: std::collections::HashMap<String, String>,
    },
    #[serde(rename_all = "camelCase")]
    TimeBoundary {
        data_source: DataSource,
        bound: Option<TimeBoundType>,
        filter: Option<Filter>,
        context: std::collections::HashMap<String, String>,
    },
    #[serde(rename_all = "camelCase")]
    SegmentMetadata {
        data_source: DataSource,
        intervals: Vec<String>,
        to_include: ToInclude,
        merge: bool,
        analysis_types: Vec<AnalysisType>,
        lenient_aggregator_merge: bool,
    },
    #[serde(rename_all = "camelCase")]
    DataSourceMetadata {
        data_source: DataSource,
        context: std::collections::HashMap<String, String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum ToInclude {
    All,
    None,
    List(Vec<String>)
}

pub struct GroupByBuilder {
    data_source: DataSource,
    dimensions: Vec<Dimension>,
    limit_spec: Option<LimitSpec>,
    having: Option<HavingSpec>,
    granularity: Granularity,
    filter: Option<Filter>,
    aggregations: Vec<Aggregation>,
    post_aggregations: Vec<PostAggregation>,
    intervals: Vec<String>,
    subtotal_spec: Vec<Vec<String>>,
    context: std::collections::HashMap<String, String>,
}

impl GroupByBuilder {
    pub fn new(data_source: DataSource) -> Self {
        GroupByBuilder {
            data_source: data_source,
            dimensions: vec![],
            limit_spec: None,
            having: None,
            granularity: Granularity::All,
            filter: None,
            aggregations: vec![],
            post_aggregations: vec![],
            intervals: vec![],
            subtotal_spec: vec![],
            context: std::collections::HashMap::new(),
        }
    }
    pub fn dimensions(mut self, dimensions: Vec<Dimension>) -> Self {
        self.dimensions = dimensions;
        self
    }
    pub fn limit(mut self, limit: LimitSpec) -> Self {
        self.limit_spec = Some(limit);
        self
    }
    pub fn having(mut self, having: HavingSpec) -> Self {
        self.having = Some(having);
        self
    }
    pub fn granularity(mut self, granularity: Granularity) -> Self {
        self.granularity = granularity;
        self
    }
    pub fn filter(mut self, filter: Filter) -> Self {
        self.filter = Some(filter);
        self
    }
    pub fn aggregations(mut self, aggr: Vec<Aggregation>) -> Self {
        self.aggregations = aggr;
        self
    }
    pub fn post_aggregations(mut self, aggr: Vec<PostAggregation>) -> Self {
        self.post_aggregations = aggr;
        self
    }
    pub fn intervals(mut self, intervals: Vec<&str>) -> Self {
        self.intervals = intervals.iter().map(|s|s.to_string()).collect();
        self
    }
    pub fn subtotal_spec(mut self, subtotals: Vec<Vec<String>>) -> Self {
        self.subtotal_spec = subtotals;
        self
    }
    pub fn context(mut self, context: std::collections::HashMap<String,String>) -> Self {
        self.context = context;
        self
    }

    pub fn build(mut self) -> Query {
        Query::GroupBy {
            data_source: self.data_source,
            dimensions: self.dimensions,
            limit_spec: self.limit_spec,
            having: self.having,
            granularity: self.granularity,
            filter: self.filter,
            aggregations: self.aggregations,
            post_aggregations: self.post_aggregations,
            intervals: self.intervals,
            subtotal_spec: self.subtotal_spec,
            context: self.context,
            
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum HllType {
    HLL_4,
    HLL_6,
    HLL_8,
}
#[rustfmt::skip]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum Aggregation {
    Count { name: String },
    #[serde(rename_all = "camelCase")]
    LongSum { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    DoubleSum { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatSum { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    LongMax { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    DoubleMax { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatMax { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    LongMin { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatMin { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    DoubleMin { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    LongFirst { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatFirst { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    DoubleFirst { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    LongLast { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatLast { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    DoubleLast { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    StringFirst { name: String, field_name: String, max_string_bytes: usize },
    #[serde(rename_all = "camelCase")]
    StringLast { name: String, field_name: String, max_string_bytes: usize },

    #[serde(rename_all = "camelCase")]
    DoubleAny { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FloatAny { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    LongAny { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    StringAny { name: String, field_name: String },

    #[serde(rename_all = "camelCase")]
    Javascript { name: String, field_names: Vec<String>, fn_aggregate: String, fn_combine: String, fn_reset: String},
    
    #[serde(rename_all = "camelCase")]
    ThetaSketch {name: String, field_name: String, is_input_theta_sketch: bool, size: usize},


    #[serde(rename_all = "camelCase")]
    HLLSketchBuild { name: String, field_name: String, lg_k: usize, lgt_hll_type: HllType, round: bool},

    #[serde(rename_all = "camelCase")]
    Cardinality { name: String, fields: Vec<String>, by_row: bool, round: bool},

    #[serde(rename_all = "camelCase")]
    HyperUnique { name: String, field_name: String, is_input_hyper_unique: bool, round: bool},

    Filtered { filter: Filter, aggregator: Box<Aggregation>}
}

// todo: macro
impl Aggregation {
    pub fn count(name: &str) -> Aggregation {
        Aggregation::Count {
            name: name.to_string(),
        }
    }
    pub fn long_sum(name: &str, field_name: &str) -> Aggregation {
        Aggregation::LongSum {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn double_sum(name: &str, field_name: &str) -> Aggregation {
        Aggregation::DoubleSum {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn float_sum(name: &str, field_name: &str) -> Aggregation {
        Aggregation::FloatSum {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn long_max(name: &str, field_name: &str) -> Aggregation {
        Aggregation::LongMax {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn double_max(name: &str, field_name: &&str) -> Aggregation {
        Aggregation::DoubleMax {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn float_max(name: &str, field_name: &str) -> Aggregation {
        Aggregation::FloatMax {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn long_min(name: &str, field_name: &str) -> Aggregation {
        Aggregation::LongMin {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn float_min(name: &str, field_name: &str) -> Aggregation {
        Aggregation::FloatMin {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn double_min(name: &str, field_name: &str) -> Aggregation {
        Aggregation::DoubleMin {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn long_first(name: &str, field_name: &str) -> Aggregation {
        Aggregation::LongFirst {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn float_first(name: &str, field_name: &str) -> Aggregation {
        Aggregation::FloatFirst {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    // pub fn double_first(name: &str, field_name: &str) -> Aggregation {}
    // pub fn long_last(name: &str, field_name: &str) -> Aggregation {}
    // pub fn float_last(name: &str, field_name: &str) -> Aggregation {}
    // pub fn double_last(name: &str, field_name: &str) -> Aggregation {}
    // pub fn string_first(name: &str, field_name: &str, max_string_bytes: usize) -> Aggregation {}
    // pub fn string_last(name: &str, field_name: &str, max_string_bytes: usize) -> Aggregation {}
    // pub fn double_any(name: &str, field_name: &str) -> Aggregation {}
    // pub fn float_any(name: &str, field_name: &str) -> Aggregation {}
    // pub fn long_any(name: &str, field_name: &str) -> Aggregation {}
    // pub fn string_any(name: &str, field_name: &str) -> Aggregation {}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AnalysisType {
    Cardinality,
    Minmax,
    Size,
    Interval,
    TimestampSpec,
    QueryGranularity,
    Aggregators,
    Rollup,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum TimeBoundType {
    MaxTime,
    MinTime,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum SearchQuerySpec {
    #[serde(rename_all = "camelCase")]
    InsensitiveContains { value: String },
    #[serde(rename_all = "camelCase")]
    Fragment {
        case_sensitive: bool,
        values: Vec<String>,
    },
    #[serde(rename_all = "camelCase")]
    Contains { case_sensitive: bool, value: String },
    #[serde(rename_all = "camelCase")]
    Regex { pattern: String },
}

impl SearchQuerySpec {
    pub fn contains_insensitive(value: &str) -> Self {
        SearchQuerySpec::InsensitiveContains {
            value: value.to_string(),
        }
    }

    pub fn constain(value: &str, case_sensitive: bool) -> Self {
        SearchQuerySpec::Contains {
            value: value.to_string(),
            case_sensitive: case_sensitive,
        }
    }
    pub fn fragment(values: Vec<&str>, case_sensitive: bool) -> Self {
        SearchQuerySpec::Fragment {
            values: values.iter().map(|s| s.to_string()).collect(),
            case_sensitive: case_sensitive,
        }
    }

    pub fn regrex(pattern: &str) -> Self {
        SearchQuerySpec::Regex {
            pattern: pattern.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PostAggregation {
    #[serde(rename_all = "camelCase")]
    Arithmetic {
        name: String,
        Fn: String,
        fields: Vec<PostAggregator>,
        ordering: Option<String>,
    },
    DoubleGreatest {
        name: String,
        fields: Vec<PostAggregation>,
    },
    LongGreatest {
        name: String,
        fields: Vec<PostAggregation>,
    },
    LongLeast {
        name: String,
        fields: Vec<PostAggregation>,
    },
    DoubleLeast {
        name: String,
        fields: Vec<PostAggregation>,
    },
    #[serde(rename_all = "camelCase")]
    Javascript {
        name: String,
        field_names: Vec<String>,
        function: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum PostAggregator {
    #[serde(rename_all = "camelCase")]
    FieldAccess { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    FinalizingFieldAccess { name: String, field_name: String },
    #[serde(rename_all = "camelCase")]
    Constant { name: String, value: JsonAny },
    #[serde(rename_all = "camelCase")]
    HyperUniqueCardinality { field_name: String },
}

impl PostAggregator {
    pub fn field_access(name:&str, field_name: &str) -> Self {
        PostAggregator::FieldAccess {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn finalized_field_access(name:&str, field_name: &str) -> Self {
        PostAggregator::FinalizingFieldAccess {
            name: name.to_string(),
            field_name: field_name.to_string(),
        }
    }
    pub fn constant(name:&str, value: JsonAny) -> Self {
        PostAggregator::Constant {
            name: name.to_string(),
            value: value,
        }
    }
    pub fn hyper_unique_cardinality(field_name:&str) -> Self {
        PostAggregator::HyperUniqueCardinality{
            field_name: field_name.to_string(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type", rename = "default")]
pub struct LimitSpec {
    pub limit: usize,
    pub columns: Vec<OrderByColumnSpec>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderByColumnSpec {
    pub dimension: String,
    pub direction: Ordering,
    pub dimension_order: SortingOrder,
}

impl OrderByColumnSpec {
    pub fn new(dimension: &str, direction: Ordering, dimension_order: SortingOrder) -> Self {
        OrderByColumnSpec {
            dimension: dimension.to_string(),
            direction: direction,
            dimension_order: dimension_order,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ResultFormat {
    List,
    CompactedList,
    ValueVector,
}

#[rustfmt::skip]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum HavingSpec {
    Filter { filter: Filter},
    GreaterThan { aggregation: String, value: JsonNumber },
    EqualTo { aggregation: String, value: JsonNumber },
    LessThan { aggregation: String, value: JsonNumber },
    DimSelector { dimension: Dimension, value: JsonAny }, //todo
    #[serde(rename_all = "camelCase")]
    And { having_specs: Vec<HavingSpec> },
    #[serde(rename_all = "camelCase")]
    Or { having_specs: Vec<HavingSpec> },
    #[serde(rename_all = "camelCase")]
    Not { having_specs: Box<HavingSpec> },
}

impl HavingSpec {
    pub fn filter(filter: Filter) -> Self {
        HavingSpec::Filter {
            filter: filter
        }
    }
    pub fn greater_than(aggregation: &str, value: JsonNumber) -> Self {
        HavingSpec::GreaterThan {
            aggregation: aggregation.to_string(),
            value: value,
        }
    }
    pub fn equal_to(aggregation: &str, value:JsonNumber ) -> Self {
        HavingSpec::EqualTo {
            aggregation: aggregation.to_string(),
            value: value,
        }
    }
    pub fn less_than(aggregation: &str, value: JsonNumber) -> Self {
        HavingSpec::LessThan {
            aggregation: aggregation.to_string(),
            value: value,
        }
    }
}


#[serde(untagged)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum JsonNumber {
    Float(f32),
    Integer(isize)
}

impl From<f32> for JsonNumber {
    fn from(float: f32) -> Self {
        JsonNumber::Float(float)
    }
}

impl From<isize> for JsonNumber {
    fn from(integer: isize) -> Self {
        JsonNumber::Integer(integer)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum JsonAny {
    Float(f32),
    Integer(isize),
    STRING(String),
    Boolean(bool)
}

impl From<f32> for JsonAny {
    fn from(float: f32) -> Self {
        JsonAny::Float(float)
    }
}

impl From<isize> for JsonAny {
    fn from(integer: isize) -> Self {
        JsonAny::Integer(integer)
    }
}

impl From<bool> for JsonAny {
    fn from(boolean: bool) -> Self {
        JsonAny::Boolean(boolean)
    }
}

impl From<String> for JsonAny {
    fn from(str: String) -> Self {
        JsonAny::STRING(str)
    }
}

impl From<&str> for JsonAny {
    fn from(str: &str) -> Self {
        JsonAny::STRING(str.to_string())
    }
}