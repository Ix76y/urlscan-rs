use crate::UrlScanClient;
use crate::error::UrlScanError;
use crate::http::get_plain;

use reqwest::header::HeaderMap;


impl UrlScanClient {
    /// Gets the result from a previously submitted page with the UUID of the submission and returns the JSON as string
    pub fn get_result(&self, uuid: &str) -> Result<String, UrlScanError> {
        let request_url = format!("{}{}result/{}", &self.domain, &self.endpoint, uuid);
        
        get_plain(&request_url, HeaderMap::new())
    }

    /* 
    /// Get a screenshot of a previously submitted page with the UUID of the submission
    pub fn get_screenshot(&self, uuid: &str) -> Result<String, UrlScanError> {
        let request_url = format!("{}screenshots/{}.png", &self.domain, uuid);
        
        get_plain(&request_url, HeaderMap::new())
    }
    */

    /// Get the DOM of a previously submitted page with the UUID of the submission
    pub fn get_dom(&self, uuid: &str) -> Result<String, UrlScanError> {
        let request_url = format!("{}dom/{}", &self.domain, uuid);
        println!("Request: {request_url}");

        get_plain(&request_url, HeaderMap::new())
    }
}