use reqwest::{blocking::{Client, Response}, StatusCode, header::HeaderMap};
use serde::de::DeserializeOwned;

use crate::error::UrlScanError;


static DEFAULT_USER_AGENT: &str = "rust-client/urlscan+https://github.com/Ix76y/urlscan-rs";

fn process_response<T>(response: Response) -> Result<T, UrlScanError> where T: DeserializeOwned {
    let status = response.status();

    match status {
        StatusCode::OK => Ok(response.json()?),
        _ => Err(UrlScanError::Unknown),
    }
}

pub(crate) fn get<T>(url: &str, headers: HeaderMap) -> Result<T, UrlScanError> where T: DeserializeOwned {
    let client = Client::builder().user_agent(DEFAULT_USER_AGENT).default_headers(headers).build()?;
    let response = client.get(url).send()?;
    process_response(response)
}

pub(crate) fn post<T>(url: &str, headers: HeaderMap, body: String) -> Result<T, UrlScanError> where T: DeserializeOwned {
    let client: Client = Client::builder().user_agent(DEFAULT_USER_AGENT).default_headers(headers).build()?;
    let response: Response = client.post(url).body(body).send()?;
    process_response(response)
}