pub async fn get_public_ip() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://api.ipify.org").await?.text().await?;

    Ok(response)
}
