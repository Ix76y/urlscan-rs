pub mod api;

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
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}