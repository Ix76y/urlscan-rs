use core::fmt;
use std::collections::HashMap;
use url::Url;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    message: String,
    uuid: String,
    result: Url,
    api: Url,
    visibility: String,
    options: HashMap<String, String>,
    url: Url,
    country: String,
}

impl fmt::Display for Submission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}