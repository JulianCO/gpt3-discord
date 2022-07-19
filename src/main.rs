pub mod openai;

use discord::{
    model::{Event, Message},
    Discord,
};
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client,
};
use serde_json::to_string_pretty;
use std::env;

use openai::{GPTParameters, GPTResponse};

static OPENAI_URL: &str = "https://api.openai.com/v1/completions";

enum IncomingRequest {
    Ignore,
    OpenAIComplete { prompt: String },
}

fn parse_message(message: &Message) -> IncomingRequest {
    let message_text = message.content.as_str();
    if message_text.starts_with("!gpt3 ") {
        IncomingRequest::OpenAIComplete {
            prompt: message_text.split_at(6).1.to_owned(),
        }
    } else {
        IncomingRequest::Ignore
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai_key = env::var("OPENAI_KEY")
        .expect("Set the environment variable OPENAI_KEY to use for this bot");

    let discord_token = env::var("DISCORD_TOKEN")
        .expect("Set the environment variable DISCORD_TOKEN to use for this bot");

    let discord_api = Discord::from_bot_token(&discord_token)?;

    let (mut discord_conn, _) = discord_api.connect()?;

    let client = Client::new();

    loop {
        match discord_conn.recv_event() {
            Ok(Event::MessageCreate(message)) => match parse_message(&message) {
                IncomingRequest::OpenAIComplete { prompt } => {
                    let request_parameters = GPTParameters {
                        model: "text-davinci-002".to_string(),
                        prompt: prompt,
                        temperature: 0.7,
                        max_tokens: 50,
                    };
                    let request = client
                        .post(OPENAI_URL)
                        .header(CONTENT_TYPE, "application/json")
                        .header(AUTHORIZATION, format!("Bearer {}", openai_key))
                        .json(&request_parameters);
                    let res: GPTResponse = request.send().await?.json().await?;

                    let pretty_printed = to_string_pretty(&res)?;

                    let _ = discord_api.send_message(
                        message.channel_id,
                        &format!("```{pretty_printed}```"),
                        "",
                        false,
                    );
                }
                IncomingRequest::Ignore => {}
            },
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }

    Ok(())
}
