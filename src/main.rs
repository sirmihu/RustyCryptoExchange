use reqwest;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let api_key = "*";

    let base_url = "https://rest.coinapi.io";
    let asset_id_base = "BTC";

    let url = format!("{}/v1/exchangerate/{}", base_url, asset_id_base);

    let client = reqwest::Client::new();

    let response = client.get(&url)
        .header("X-CoinAPI-Key", api_key)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.bytes().await?;
        let body_str = String::from_utf8_lossy(&body);

        let result: serde_json::Value = serde_json::from_str(&body_str)?;

        println!("{:?}", result);
    } else {
        println!("Błąd zapytania: {:?}", response.status());
    }

    Ok(())
}
