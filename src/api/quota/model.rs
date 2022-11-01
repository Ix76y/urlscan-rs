use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quota {
    pub source: String,
    pub limits: Limits,
}

impl fmt::Display for Quota {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Quota Source: {}\nQueryable Fields: {:?}\nQuery Visibility: {:?}", self.source, self.limits.queryable_fields, self.limits.query_visibility)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Limits {
    pub private: Category,
    pub public: Category,
    pub retrieve: Category,
    pub search: Category,
    pub unlisted: Category,
    pub livescan: Category,
    pub liveshot: Liveshot,
    #[serde(rename = "maxRetentionPeriodDays")]
    pub max_retention_period_days: u32,
    #[serde(rename = "maxSearchRangeMonths")]
    pub max_search_range_months: u32,
    #[serde(rename = "maxSearchResults")]
    pub max_search_results: u32,
    pub products: Vec<String>,
    pub features: Vec<String>,
    #[serde(rename = "queryableFields")]
    pub queryable_fields: Vec<String>,
    #[serde(rename = "queryVisibility")]
    pub query_visibility: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub day: Limit,
    pub hour: Limit,
    pub minute: Limit, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Limit {
    pub limit: u32,
    pub used: u32,
    pub remaining: u32,
    pub percent: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Liveshot {
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
}