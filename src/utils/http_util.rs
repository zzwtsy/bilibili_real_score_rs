///# 发送 http get 请求
#[tokio::main]
pub async fn send_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp)
}
