use core::fmt;
use std::collections::HashMap;
use url::Url;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    pub message: String,
    pub uuid: String,
    pub result: Url,
    pub api: Url,
    pub visibility: String,
    pub options: HashMap<String, String>,
    pub url: Url,
    pub country: String,
}

impl fmt::Display for Submission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. \nUUID: {}\nView Result: {}", self.message, self.uuid, self.result)
    }
}