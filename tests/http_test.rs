use chrono::{Duration, Local};
use reqwest::Client;
use std::collections::HashMap;

#[test]
fn out() {
    let now = Local::now();
    println!("{} {}", now.timestamp(), now.naive_local());
    let exp = now + Duration::hours(2);
    println!("{} {}", exp.timestamp(), exp.naive_local());
}

#[tokio::test]
async fn get() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .pool_idle_timeout(std::time::Duration::from_secs(30))
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
