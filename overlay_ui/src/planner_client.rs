use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct PlannerClient {
    base_url: String,
}

#[derive(Serialize)]
struct ChatRequest {
    message: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    response: String,
    status: String,
}

impl PlannerClient {
    pub fn new() -> Self {
        Self {
            base_url: "http://localhost:5000".to_string(),
        }
    }

    pub async fn send_chat(&self, message: String) -> Result<String, String> {
        let client = reqwest::Client::new();
        let request = ChatRequest { message };
        
        match client
            .post(format!("{}/chat", self.base_url))
            .json(&request)
            .send()
            .await
        {
            Ok(response) => {
                match response.json::<ChatResponse>().await {
                    Ok(chat_resp) => Ok(chat_resp.response),
                    Err(e) => Err(format!("Failed to parse response: {}", e)),
                }
            }
            Err(e) => Err(format!("Failed to send request: {}", e)),
        }
    }
}