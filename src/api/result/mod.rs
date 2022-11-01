use crate::UrlScanClient;
use crate::error::UrlScanError;
use crate::http::{get, get_plain};

pub mod model;

use reqwest::header::{HeaderMap, HeaderValue};


impl UrlScanClient {
    pub fn get_result(&self, uuid: &str) -> Result<String, UrlScanError> {
        let request_url = format!("{}{}result/{}", &self.domain, &self.endpoint, uuid);
        
        get::<String>(&request_url, HeaderMap::new())
    }

    pub fn get_screenshot(&self, uuid: &str) -> Result<String, UrlScanError> {
        let request_url = format!("{}screenshots/{}.png", &self.domain, uuid);
        
        get::<String>(&request_url, HeaderMap::new())
    }


    pub fn get_dom(&self, uuid: &str) -> Result<String, UrlScanError> {
        let request_url = format!("{}dom/{}", &self.domain, uuid);
        println!("Request: {request_url}");

        let mut headers = HeaderMap::new();
        headers.insert("Accept-encoding", HeaderValue::from_str("gzip").unwrap());

        get_plain(&request_url, headers)
    }
}