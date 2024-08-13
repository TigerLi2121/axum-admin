use std::{collections::HashMap, time::Duration};

use reqwest::Client;
use sqlx::types::chrono::Local;

#[test]
fn out() {
    let dt = Local::now();
    println!("{}", dt.naive_local())
}

#[tokio::test]
async fn get() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(10)
        .build()?;
    let response = client.get("https://httpbin.org/ip").send().await?;
    println!("{}", response.text().await?);

    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{:#?}", resp);
    Ok(())
}

#[tokio::test]
async fn get_json() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
