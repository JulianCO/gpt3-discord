use reqwest::{header::{AUTHORIZATION, CONTENT_TYPE}, Client};
use std::env;

static OPENAI_URL: &str = "https://api.openai.com/v1/completions";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("OPENAI_KEY")
        .expect("Set the environment variable OPENAI_KEY to use for this bot");

    let client = Client::new();

    let request = client
        .post(OPENAI_URL)
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", api_key))
        .body(r#"{"model": "text-davinci-002", "prompt": "Say this is a test", "temperature": 0, "max_tokens": 6}"#);
    
        println!("Request: {:?}", &request);
        
    let res = request.send().await?;

    println!("Response: {:?}", &res.text().await?);

    Ok(())
}
