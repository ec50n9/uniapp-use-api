use actix_web::{post, web, Error, HttpResponse};
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const OPENAI_BASE_URL: &str = "https://api.deepseek.com";
const OPENAI_API_KEY: &str = "sk-0d5ac1a4720e4ae48c1f0885c7824f40";
const OPENAI_MODEL: &str = "deepseek-chat";

const TEACHER_PROMPT: &str = include_str!("../openai_prompts/english.txt");

#[derive(Debug, Deserialize)]
pub struct OpenAiRequest {
    text: String,
}

#[derive(Deserialize, Serialize)]
struct OpenAiResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    error: Option<String>,
    choices: Vec<Choice>,
}

#[derive(Deserialize, Serialize)]
struct Choice {
    index: u64,
    logprobs: Option<u64>,
    finish_reason: String,
    message: Message,
}

#[derive(Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[post("/openai")]
pub async fn openai(req: web::Json<OpenAiRequest>) -> Result<HttpResponse, Error> {
    let client = Client::new();
    let url = format!("{}/v1/chat/completions", OPENAI_BASE_URL);

    let prompt = TEACHER_PROMPT;

    let request_body = serde_json::json!({
        "model": &OPENAI_MODEL,
        "messages": [
            {"role": "system","content": prompt},
            {"role": "user","content": format!("Please analyze the following sentence: [{}]", req.0.text)}
        ],
        "stream": true,
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", OPENAI_API_KEY))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let stream = response.bytes_stream().map(|chunk| {
        chunk
            .map_err(|err| actix_web::error::ErrorInternalServerError(err))
            .and_then(|data| {
                if let Ok(text) = String::from_utf8(data.to_vec()) {
                    let processed = text
                        .split('\n')
                        .filter(|line| line.starts_with("data: "))
                        .map(|line| line[6..].to_string())
                        .collect::<String>();

                    if processed == "[DONE]" {
                        return Ok::<web::Bytes, actix_web::error::Error>(web::Bytes::from("\n"));
                    }

                    // Parse the JSON response
                    if let Ok(json) = serde_json::from_str::<Value>(&processed) {
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            if !content.eq("```") && !content.eq("json") {
                                return Ok::<web::Bytes, actix_web::error::Error>(
                                    web::Bytes::from(content.to_string()),
                                );
                            }
                        }
                    }
                }
                Ok::<web::Bytes, actix_web::error::Error>(web::Bytes::new())
            })
    });

    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(stream))
}
