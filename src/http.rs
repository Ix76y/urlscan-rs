use reqwest::{blocking::{Client, Response}, StatusCode, header::HeaderMap};
use serde::de::DeserializeOwned;
use std::io::Read;
use libflate::gzip::Decoder;
use bytes::Bytes;

use crate::error::UrlScanError;


static DEFAULT_USER_AGENT: &str = "rust-client/urlscan+https://github.com/Ix76y/urlscan-rs";

fn process_response<T>(response: Response) -> Result<T, UrlScanError> where T: DeserializeOwned {
    let status = response.status();

    match status {
        StatusCode::OK => Ok(response.json()?),
        _ => Err(UrlScanError::Unknown),
    }
}


fn process_response_plain(response: Response) -> Result<String, UrlScanError> {
    let status = response.status();
    let headers = response.headers();
    match headers.get("content-encoding") {
        Some(enc) => {
            if enc == "gzip" {
                let mut decoder = Decoder::new(response).unwrap();
                let mut content = String::new();
                // let mut content = Vec::new();
                decoder.read_to_string(&mut content).unwrap();
                // decoder.read_to_end(&mut content).unwrap();
                return Ok(content);
            } else {
                println!("Unknown encoding of content: {:?}", enc);
            }
        },
        _ => {},
    }

    match status {
        StatusCode::OK => Ok(response.text_with_charset("utf-8")?),
        _ => Err(UrlScanError::Unknown),
    }
}

pub(crate) fn get<T>(url: &str, headers: HeaderMap) -> Result<T, UrlScanError> where T: DeserializeOwned {
    let client = Client::builder().user_agent(DEFAULT_USER_AGENT).default_headers(headers).build()?;
    let response = client.get(url).send()?;
    process_response(response)
}

pub(crate) fn get_plain(url: &str, headers: HeaderMap) -> Result<String, UrlScanError> {
    let client = Client::builder().user_agent(DEFAULT_USER_AGENT).default_headers(headers).build()?;
    let response = client.get(url).send()?;
    process_response_plain(response)
}

pub(crate) fn get_as_bytes(url: &str, headers: HeaderMap) -> Result<Bytes, UrlScanError> {
    let client = Client::builder().user_agent(DEFAULT_USER_AGENT).default_headers(headers).build()?;
    let response = client.get(url).send()?;
    let status = response.status();

    match status {
        StatusCode::OK => Ok(response.bytes()?),
        _ => Err(UrlScanError::Unknown),
    }
}

pub(crate) fn post<T>(url: &str, headers: HeaderMap, body: String) -> Result<T, UrlScanError> where T: DeserializeOwned {
    let client: Client = Client::builder().user_agent(DEFAULT_USER_AGENT).default_headers(headers).build()?;
    let response: Response = client.post(url).body(body).send()?;
    process_response(response)
}