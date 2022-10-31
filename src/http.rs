use std::collections::HashMap;

use reqwest::{Client, Response, StatusCode };
use serde::de::DeserializedOwned;


fn process_response(response: Response) -> Result<T> where T: DeserializedOwned {
    let status = response.status();
    match status {
        StatusCode::Ok => Ok(resp.json()?),
        _ => Err((status, resp.text()?).into()),
    }
}

pub(crate) fn get(url: &str, headers: HashMap<String, String>, user_agent: &str) -> Result<T> where T: DeserializedOwned {
    let client = Client::builder().user_agent(user_agent).build()?;
    
    for (key, value) in &headers {
        client.header(key, value)
    }

    let response = client.get(url).send()?;
    process_response(response)
}