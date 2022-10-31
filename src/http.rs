use reqwest::{blocking::{Client, Response}, StatusCode, Error, header::HeaderMap };
// use serde::de::DeserializeOwned;


static DEFAULT_USER_AGENT: &str = "rust-client/urlscan+https://github.com/Ix76y/urlscan-rs";

fn process_response(response: Response) -> Result<String, Error> {
    // let status = response.status();
    response.text()
}

pub(crate) fn get(url: &str, headers: HeaderMap) -> Result<String, Error> {
    let client = Client::builder().user_agent(DEFAULT_USER_AGENT).default_headers(headers).build()?;
    let response = client.get(url).send()?;
    process_response(response)
}