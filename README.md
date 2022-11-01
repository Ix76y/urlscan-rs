# urlscan-rs
Rust wrapper for URLScan.io API

## Examples
Get your current quota with limits:
```Rust
let client = UrlScanClient::new("YOUR-API-KEY-HERE");
let response = client.get_quota();
match response {
    Ok(quota) => println!("{}", quota),
    _ => println!("We got an error..."),
}
```

## FAQs
- How do I get a URLScan.io API Key? 
Create an URLScan.io account and then go to [Settings & API](https://urlscan.io/user/profile/) to get your API key.
- Is the URLScan.io API key free? Yes, URLScan.io has free API keys that are limited by time. If you reach the limit you can pay for unlimited requests.


**License**: MIT