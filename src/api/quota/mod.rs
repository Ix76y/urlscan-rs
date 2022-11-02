use crate::UrlScanClient;
use crate::error::UrlScanError;

/// Quota model contains the struct for quota
pub mod model;
pub use model::Quota;

// use std::collections::HashMap;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::http::get;

impl UrlScanClient {
    /// Get your current quota to check your limits and the amount of requests you already made
    /// 
    /// ## Example usage
    /// ```rust
    /// let client = UrlScanClient::new("YOUR-API-KEY-HERE");
    /// let response = client.get_quota();
    /// match response {
    ///     Ok(quota) => println!("{}", quota),
    ///     _ => println!("We got an error..."),
    /// }
    /// ```
    pub fn get_quota(&self) -> Result<Quota, UrlScanError> {
        let url = format!("{}user/quotas/", &self.domain);
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
        headers.insert("API-Key", HeaderValue::from_str(&self.api_key).unwrap());

        get::<Quota>(&url, headers)
        //println!("Response: {:?}", response);
    }
}