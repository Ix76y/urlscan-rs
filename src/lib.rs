//! # URLScan.io REST API Wrapper.
//!
//! Provides an abstraction over the URLScan.io API. 
//! This library supports the following tasks[^note]:
//! - [x] Get Quota
//! - [x] Submit URL to be scanned
//! - [ ] Get Result of scan
//! - [x] Get DOM of previously scanner URL by UUID
//! - [ ] Get Screenshot of page
//! - [ ] Search functionality
//!
//! [^note]: Tasks that are not marked as complete are currently being worked on and will be part of a future release.
//! 
//! ## API Key
//! In order to use this library, you need an URLScan.io API key.
//! To get an API key, create an URLScan.io account and then go to [Settings & API](https://urlscan.io/user/profile/) to get your API key.
//! 
//! ## Examples
//! Get your current quota with limits:
//! ```Rust
//! // Create a URlScan Client with you API key
//! let client = UrlScanClient::new("YOUR-API-KEY-HERE");
//! 
//! // Get the current quota for your API key
//! let response = client.get_quota();
//! match response {
//!     Ok(quota) => println!("{}", quota),
//!     _ => println!("We got an error..."),
//! }
//! ```
//! 
#![warn(missing_docs)]

pub mod api;
mod http;
mod error;

#[derive(Clone)]
pub struct UrlScanClient {
    api_key: String,
    domain: String,
    endpoint: String,
}

impl UrlScanClient {
    pub fn new(api_key: &str) -> Self {
        //! Creates a new URLScan.io API client 
        //! 
        //! ## Example usage
        //! ```rust
        //! use urlscan::UrlScanClient;
        //! let urlscan_client = UrlScanClient::new("API_KEY");
        //! ```
        UrlScanClient {
            api_key: api_key.into(),
            domain: "https://urlscan.io/".into(),
            endpoint: "api/v1/".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::UrlScanClient;

    static API_KEY: &str = "TODO";
    static UUID: &str = "20d16cb9-72f1-4139-bd67-130e0bc02da8"; // crates.io scan


    #[test]
    fn test_client() {
        let client = UrlScanClient::new(API_KEY);
        assert_eq!(client.domain, "https://urlscan.io/");
        assert_eq!(client.endpoint, "api/v1/");
    }

    #[test]
    fn test_dom() {
        let client = UrlScanClient::new(API_KEY);
        let response = client.get_dom(UUID);
        match response {
            Ok(dom) => println!("{}", dom),
            _ => println!("We got an error..."),
        }
    }
}
