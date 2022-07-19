use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Value};
use std::env;

static OPENAI_URL: &str = "https://api.openai.com/v1/completions";

#[derive(Serialize, Deserialize)]
struct GPTParameters {
    model: String,
    prompt: String,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct GPTResponse {
    choices: Vec<GPTChoice>,
    usage: GPTUsageStats,
}

#[derive(Serialize, Deserialize)]
struct GPTChoice {
    text: String,
    index: u32,
    finish_reason: String,
}

#[derive(Serialize, Deserialize)]
struct GPTUsageStats {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_KEY")
        .expect("Set the environment variable OPENAI_KEY to use for this bot");

    let client = Client::new();

    let request_parameters = GPTParameters {
        model: "text-davinci-002".to_string(),
        prompt: "Tell me about your favorite author".to_string(),
        temperature: 0.7,
        max_tokens: 15,
    };

    let request = client
        .post(OPENAI_URL)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&request_parameters);

    let res: GPTResponse = request.send().await?.json().await?;

    println!("Response: {}", to_string_pretty(&res)?);

    Ok(())
}
