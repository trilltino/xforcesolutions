use gloo_net::http::Request;
use serde_json::Value;

pub struct ApiClient;

impl ApiClient {
    pub async fn post(endpoint: &str, data: Value) -> Result<String, String> {
        let response = Request::post(endpoint)
            .header("content-type", "application/json")
            .json(&data)
            .map_err(|e| format!("Request error: {e}"))?
            .send()
            .await
            .map_err(|e| format!("Network error: {e}", ))?;

        if response.ok() {
            response.text().await.map_err(|e| format!("Response parse error: {e}",))
        } else {
            Err(format!("HTTP error: {}", response.status()))
        }
    }
}