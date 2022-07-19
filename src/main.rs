use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};
use std::env;

static OPENAI_URL: &str = "https://api.openai.com/v1/completions";

#[derive(Serialize, Deserialize)]
struct GPTParameters {
    model: String,
    prompt: String,
    temperature: u32,
    max_tokens: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_KEY")
        .expect("Set the environment variable OPENAI_KEY to use for this bot");

    let client = Client::new();

    let request_parameters = GPTParameters {
        model: "text-davinci-002".to_string(),
        prompt: "Say this is a test".to_string(),
        temperature: 0,
        max_tokens: 6,
    };

    let request = client
        .post(OPENAI_URL)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&request_parameters);

    println!("Request: {:?}", &request);

    let res = request.send().await?;

    println!("Response: {:?}", &res.text().await?);

    Ok(())
}
