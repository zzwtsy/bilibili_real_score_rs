///## 发送 http get 请求
#[tokio::main]
pub async fn send_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
    .get(url)
    .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36")
    .send()
    .await?
    .text()
    .await?;
    Ok(resp)
}
