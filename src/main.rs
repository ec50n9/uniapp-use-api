mod prompt_util;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use prompt_util::generate_response;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const OPENAI_BASE_URL: &str = "https://api.deepseek.com";
const OPENAI_API_KEY: &str = "sk-0d5ac1a4720e4ae48c1f0885c7824f40";
const OPENAI_MODEL: &str = "deepseek-chat";

const TEACHER_PROMPT: &str = include_str!("openai_prompts/english.txt");

#[derive(Debug, Deserialize)]
struct OpenAiRequest {
    text: String,
    analysis: Option<bool>,
    corrections: Option<bool>,
    learning_diagnosis: Option<bool>,
    advancement_suggestions: Option<bool>,
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
async fn openai(req: web::Json<OpenAiRequest>) -> impl Responder {
    let client = Client::new();
    let url = format!("{}/v1/chat/completions", OPENAI_BASE_URL);

    let prompt = generate_response(
        req.analysis.unwrap_or(false),
        req.corrections.unwrap_or(false),
        req.learning_diagnosis.unwrap_or(false),
        req.advancement_suggestions.unwrap_or(false),
    );

    let request_body = serde_json::json!({
        "model": &OPENAI_MODEL,
        "messages": [
            {"role": "system","content": prompt},
            {"role": "user","content": format!("Please analyze the following sentence: [{}]", req.0.text)}
        ],
        "stream": false,
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", OPENAI_API_KEY))
        .json(&request_body)
        .send()
        .await;

    let response = match response {
        Ok(res) => res,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let response_body = match response.json::<OpenAiResponse>().await {
        Ok(body) => body,
        Err(err) => return HttpResponse::InternalServerError().json(
            serde_json::json!({ "error": "Failed to parse response", "detail": err.to_string() }),
        ),
    };

    if let Some(error) = response_body.error {
        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"error": "OpenAI error", "detail": error}));
    }

    let content = response_body
        .choices
        .first()
        .unwrap()
        .message
        .content
        .clone();

    println!("content: {:?}", content);

    HttpResponse::Ok().json(serde_json::json!({ "content": content }))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(openai)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
