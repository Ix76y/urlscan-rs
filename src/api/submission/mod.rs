use crate::UrlScanClient;

impl UrlScanClient {
    pub fn scan_url(&self, url: &str) {
        let request_url = format!("{}{}scan/", &self.domain, &self.endpoint);
    }

    pub fn scan_urls(&self, urls: Vec<String>) {

    }
}