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

    #[test]
    fn test_quota() {
        let api_key = "TODO";
        let client = UrlScanClient::new(api_key);
        let response = client.get_quota();
        match response {
            Ok(quota) => println!("{}", quota),
            _ => println!("We got an error..."),
        }
    }
}
