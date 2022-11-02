use crate::{UrlScanClient, error::UrlScanError, http::post};

/// Submission model containing information about the successfull submission
pub mod model;
pub use model::Submission;
use reqwest::header::{HeaderMap, HeaderValue};

impl UrlScanClient {
    /// Scans a given URL with a specific visibility.
    /// The URLScan.io API has different request limits of visibility, more for public scans then for private ones.
    /// However, for security researchers it is recommend to use a private visibility when investigating potentially malicious URLs.
    pub fn scan_url(&self, url: &str, visibility: &str, _tags: Vec<String>) -> Result<Submission, UrlScanError> {
        let request_url = format!("{}{}scan/", &self.domain, &self.endpoint);

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_str("application/json").unwrap());
        headers.insert("API-Key", HeaderValue::from_str(&self.api_key).unwrap());

        let body = format!("{{\"url\": \"{}\", \"visibility\": \"{}\", \"tags\": [] }}", url, visibility);

        post::<Submission>(&request_url, headers, body)
    }

    /*pub fn scan_urls(&self, urls: Vec<String>) -> Vec<Result<Submission, UrlScanError>> {
        let mut results = Vec::new();
        for url in urls {
            let result = self.scan_url(&url);
            results.push(result);
        }
        results
    }*/
}