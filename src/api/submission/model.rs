use core::fmt;
use std::collections::HashMap;
use url::Url;

use serde::{Deserialize, Serialize};

/// Submission struct with information for a successful submission
#[derive(Debug, Serialize, Deserialize)]
pub struct Submission {
    /// Submission message, usually information if the submission was successful
    pub message: String,
    /// UUID of the submission.
    /// This should be used for further investigations when querying the result of a scan.
    pub uuid: String,
    /// Link to the submission for use in Browser 
    pub result: Url,
    /// Api URL to query the result of the scan.
    /// *Note:* The result is usually available after a short amount of time. It is recommneded to wait 10s before querying the result.
    pub api: Url,
    /// Visibility used for the scan.
    pub visibility: String,
    /// Any options provided for the scan, this can be:
    /// - customagent: Override User-Agent for this scan
    /// - referer: Override HTTP referer for this scan
    /// - visibility: One of public, unlisted, private. Defaults to your configured default visibility.
    /// - tags: User-defined tags to annotate this scan, e.g. "phishing" or "malicious". Limited to 10 tags.
    /// - overrideSafety: If set to any value, this will disable reclassification of URLs with potential PII in them. Use with care!
    /// - country: Specify which country the scan should be performed from (2-Letter ISO-3166-1 alpha-2 country). See the API endpoint for possible values.
    pub options: HashMap<String, String>,
    /// URL that was submitted to be scanned
    pub url: Url,
    /// Country that was used as origin for the scan
    pub country: String,
}

impl fmt::Display for Submission {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}. \nUUID: {}\nView Result: {}", self.message, self.uuid, self.result)
    }
}