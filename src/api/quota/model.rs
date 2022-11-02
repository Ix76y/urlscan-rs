use core::fmt;

use serde::{Deserialize, Serialize};

/// Quota struct contains the source and a limits object
#[derive(Debug, Serialize, Deserialize)]
pub struct Quota {
    /// Source of quota, usually 'user'
    pub source: String,
    /// Limits contains all limits for a specified API key
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

/// Category struct contains information about daily, hourly, and minute limit
/// where each of them is a Limit struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    /// Daily limits
    pub day: Limit,
    /// Hourly limits
    pub hour: Limit,
    /// Limits per minute
    pub minute: Limit, 
}

/// Limit struct contains the current limit, how much of it was used so far and how much of it is remaining.
#[derive(Debug, Serialize, Deserialize)]
pub struct Limit {
    /// Limit for a specific category
    pub limit: u32,
    /// How much requests have been already used
    pub used: u32,
    /// How many requests are remanining
    pub remaining: u32,
    /// Precentage of used/limit
    pub percent: f32,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Liveshot {
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
}