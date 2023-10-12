use std::collections::HashMap;

#[test]
fn out() {
    println!("{}", 123)
}

#[tokio::test]
async fn get() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
