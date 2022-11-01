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
