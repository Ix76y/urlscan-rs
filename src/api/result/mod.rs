use crate::UrlScanClient;
use crate::error::UrlScanError;
use crate::http::get_plain;
use crate::http::get_as_bytes;

use reqwest::header::HeaderMap;
use image::{DynamicImage, ImageError};
use std::io::{Error, ErrorKind};


impl UrlScanClient {
    /// Gets the result from a previously submitted page with the UUID of the submission and returns the JSON as string
    /// After submitting a URI it takes usually a few seconds before the result becomes available. It is therefore recommended to wait ~10s before querying the result.
    /// If the result is not available yet, the request will return a 404
    pub fn get_result(&self, uuid: &str) -> Result<String, UrlScanError> {
        let request_url = format!("{}{}result/{}", &self.domain, &self.endpoint, uuid);
        
        get_plain(&request_url, HeaderMap::new())
    }

    
    /// Get a screenshot of a previously submitted page with the UUID of the submission
    pub fn get_screenshot(&self, uuid: &str) -> Result<DynamicImage, ImageError> {
        let request_url = format!("{}screenshots/{}.png", &self.domain, uuid);
        let response = get_as_bytes(&request_url, HeaderMap::new());

        match response {
            Ok(response) => image::load_from_memory(&response),
            _ => Err(ImageError::IoError(Error::new(ErrorKind::Other, "oh no!"))),
        }
    }

    /// Saves an image from memory on disk
    /// the path argument should have the path and name of the file including file extension
    pub fn save_screenshot(&self, image: DynamicImage, path: &str) -> Result<(), ImageError> {
        image.save(path)
    }
    

    /// Get the DOM of a previously submitted page with the UUID of the submission
    /// After submitting a URI it takes usually a few seconds before the result becomes available. It is therefore recommended to wait ~10s before querying the result.
    /// If the result is not available yet, the request will return a 404
    pub fn get_dom(&self, uuid: &str) -> Result<String, UrlScanError> {
        let request_url = format!("{}dom/{}", &self.domain, uuid);
        println!("Request: {request_url}");

        get_plain(&request_url, HeaderMap::new())
    }
}