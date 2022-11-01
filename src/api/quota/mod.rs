use crate::UrlScanClient;
use crate::error::UrlScanError;

pub mod model;
pub use model::Quota;

// use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::http::get;

impl UrlScanClient {
    pub fn get_quota(&self) -> Result<Quota, UrlScanError> {
        let url = format!("{}user/quotas/", &self.domain);
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
        headers.insert("API-Key", HeaderValue::from_str(&self.api_key).unwrap());

        get::<Quota>(&url, headers)
        //println!("Response: {:?}", response);
    }
}