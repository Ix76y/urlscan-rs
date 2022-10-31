use crate::UrlScanClient;

use std::collections::HashMap;
use crate::http::get;

impl UrlScanClient {
    pub fn get_quota(&self) {
        let url = format!("{}user/quotas/", &self.domain);
        let mut headers = HashMap::new();

        headers.insert("Content-Type", "application/json");
        headers.insert("API-Key", &self.api_key);

        let response = get(&url, headers);
        println!("Response: {:?}", response);
    }
}