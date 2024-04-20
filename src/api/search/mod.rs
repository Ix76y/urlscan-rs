use crate::UrlScanClient;
use crate::error::UrlScanError;
use crate::http::get_plain;

use reqwest::header::HeaderMap;

impl UrlScanClient {
    /// Searches for previously submitted scans by query
    /// The search query uses the ElasticSearch syntax (URLScan.io Query Reference: https://urlscan.io/docs/search/)
    pub fn search_scans(&self, query: &str, limit: Option<i32>) -> Result<String, UrlScanError> {
        let request_url: String = match limit {
            Some(limit) => format!("{}{}search/?q={}&size={}", &self.domain, &self.endpoint, query, limit),
            None => format!("{}{}search/?q={}", &self.domain, &self.endpoint, query)
        };
        get_plain(&request_url, HeaderMap::new())
    }
}