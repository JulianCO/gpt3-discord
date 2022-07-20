pub mod openai;

use discord::{
    model::{Event, Message},
    Connection, Discord,
};
use reqwest::header::{self, AUTHORIZATION};

use reqwest::blocking::Client;
use serde_json::to_string_pretty;

use core::fmt;
use std::{env, error::Error, fs};

use openai::{GPTParameters, GPTResponse};

static OPENAI_URL: &str = "https://api.openai.com/v1/completions";

enum IncomingRequest {
    Ignore,
    OpenAIComplete { prompt: String },
}

#[derive(Debug)]
enum AppError {
    ReqwestError(reqwest::Error),
    DiscordError(discord::Error),
    SerdeJsonError(serde_json::Error),
    InvalidAuthHeader,
    Uninitialized,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ReqwestError(e) => write!(f, "error with reqwest: {e}"),
            AppError::DiscordError(e) => write!(f, "error with discord-rs: {e}"),
            AppError::Uninitialized => write!(f, "Uninitialized connections within main_loop"),
            AppError::InvalidAuthHeader => write!(f, "Problem when setting Authorization header"),
            AppError::SerdeJsonError(e) => write!(f, "error while parsing or generating json: {e}"),
        }
    }
}

impl Error for AppError {}
/*
impl From<InvalidHeaderValue> for AppError {
    fn from(_: InvalidHeaderValue) -> Self {
        AppError
    }
} */

impl From<discord::Error> for AppError {
    fn from(e: discord::Error) -> Self {
        AppError::DiscordError(e)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::ReqwestError(e)
    }
}

impl From<reqwest::header::InvalidHeaderValue> for AppError {
    fn from(_: reqwest::header::InvalidHeaderValue) -> Self {
        AppError::InvalidAuthHeader
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::SerdeJsonError(e)
    }
}

struct App {
    openai_key: String,
    discord_token: String,
    discord_api: Option<Discord>,
    discord_conn: Option<Connection>,
    openai_client: Option<Client>,
}

impl App {
    pub fn new(openai_key: &str, discord_token: &str) -> Self {
        App {
            openai_key: openai_key.to_owned(),
            discord_token: discord_token.to_owned(),
            discord_api: None,
            discord_conn: None,
            openai_client: None,
        }
    }

    fn establish_openai_connection(&mut self) -> Result<(), AppError> {
        let mut default_headers = header::HeaderMap::new();
        let mut auth_header =
            header::HeaderValue::from_str(&format!("Bearer {}", self.openai_key))?;
        auth_header.set_sensitive(true);

        default_headers.insert(AUTHORIZATION, auth_header);

        let client = Client::builder().default_headers(default_headers).build()?;

        self.openai_client = Some(client);
        Ok(())
    }

    fn establish_discord_connection(&mut self) -> Result<(), AppError> {
        let discord_api = Discord::from_bot_token(&&self.discord_token)?;

        let (discord_conn, _) = discord_api.connect()?;

        self.discord_api = Some(discord_api);
        self.discord_conn = Some(discord_conn);
        Ok(())
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

    pub fn establish_connections(&mut self) -> Result<(), AppError> {
        self.establish_openai_connection()?;
        self.establish_discord_connection()?;
        Ok(())
    }

    pub fn main_loop(&mut self) -> Result<(), AppError> {
        if let (Some(openai_client), Some(discord_api), Some(discord_conn)) = (
            &mut self.openai_client,
            &mut self.discord_api,
            &mut self.discord_conn,
        ) {
            loop {
                match discord_conn.recv_event() {
                    Ok(Event::MessageCreate(message)) => match Self::parse_message(&message) {
                        IncomingRequest::OpenAIComplete { prompt } => {
                            let request_parameters = GPTParameters::new(&prompt, 50);
                            let request = openai_client.post(OPENAI_URL).json(&request_parameters);
                            let server_res = request.send()?;
                            println!("{server_res:?}");
                            let response_text: serde_json::Value =
                                serde_json::from_str(&server_res.text()?)?;
                            println!("{}", to_string_pretty(&response_text)?);

                            let res: GPTResponse = serde_json::from_value(response_text)?;

                            let _ = discord_api.send_message(
                                message.channel_id,
                                &format!("```{}```", res.choices[0].text),
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
        } else {
            Err(AppError::Uninitialized)
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let openai_key_file = env::var("OPENAI_KEY_FILE")
        .expect("Set the environment variable OPENAI_KEY_FILE to use for this bot");

    let discord_token_file = env::var("DISCORD_TOKEN_FILE")
        .expect("Set the environment variable DISCORD_TOKEN_FILE to use for this bot");

    let openai_key = fs::read_to_string(&openai_key_file).expect(&format!(
        "Error while attempting to read OpenAI key file {openai_key_file}"
    ));

    let discord_token = fs::read_to_string(&discord_token_file).expect(&format!(
        "Error while attempting to read Discord token file {discord_token_file}"
    ));

    let mut app = App::new(&openai_key, &discord_token);
    app.establish_connections()?;

    app.main_loop()?;

    Ok(())
}
