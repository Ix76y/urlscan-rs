# urlscan-rs

[![Build Status](https://github.com/Ix76y/urlscan-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Ix76y/urlscan-rs/actions/workflows/rust.yml)
![Crates.io](https://img.shields.io/crates/v/urlscan?style=plastic)
[![GitHub license](https://img.shields.io/github/license/Ix76y/urlscan-rs?style=plastic)](https://github.com/Ix76y/urlscan-rs/blob/main/LICENSE)
![Release](https://img.shields.io/github/v/release/Ix76y/urlscan-rs?color=blue&include_prereleases&style=plastic)

## Rust wrapper for URLScan.io API

Provides an abstraction over the URLScan.io API. 
This library supports the following tasks:
- [x] Get Quota
- [x] Submit URL to be scanned
- [x] Get JSON Result of scan as String
- [x] Get DOM of previously scanner URL by UUID
- [ ] Get Screenshot of page
- [ ] Search functionality


## Examples
### Get your current quota with limits:
```rust
let client = UrlScanClient::new("YOUR-API-KEY-HERE");
let response = client.get_quota();
match response {
    Ok(quota) => println!("{}", quota),
    _ => println!("We got an error..."),
}
```

### Submitting a URL to be scanned:
```rust
let client = UrlScanClient::new(API_KEY);
let response = client.scan_url("www.url-you-want-to-check.rust", "public", vec![]);
match response {
    Ok(submission) => println!("{}", submission),
    _ => println!("We got an error..."),
}
```

**Example Output:**
```
Submission successful. 
UUID: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
View Result: https://urlscan.io/result/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/
```

### Getting the DOM:
```rust
let client = UrlScanClient::new(API_KEY);
// submit a URL to get a "submission" back or directly add the UUID:
let uuid = submission.uuid;

let response = client.get_dom(uuid);
match response {
    Ok(dom) => println!("{}", dom),
    _ => println!("There was an error, maybe scan is still running."),
}
```

## FAQs
- How do I get a URLScan.io API Key? 
Create an URLScan.io account and then go to [Settings & API](https://urlscan.io/user/profile/) to get your API key.
- Is the URLScan.io API key free? Yes, URLScan.io has free API keys that are limited by time. If you reach the limit you can pay for unlimited requests.


**License**: MIT