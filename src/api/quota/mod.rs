use crate::UrlScanClient;

// use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Error;
use crate::http::get;

impl UrlScanClient {
    pub fn get_quota(&self) -> Result<String, Error> {
        let url = format!("{}user/quotas/", &self.domain);
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
        headers.insert("API-Key", HeaderValue::from_str(&self.api_key).unwrap());

        get(&url, headers)
        //println!("Response: {:?}", response);
    }
}