use reqwest::blocking::Client;
use std::time::Duration;

use crate::announce::error::Result;

pub fn new_client() -> Result<Client> {
    let client = Client::builder()
        .user_agent("bittorrent-rust/0.1")
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(10))
        .build()?;
    Ok(client)
}
