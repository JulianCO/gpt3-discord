pub mod error;
pub mod openai;

use discord::{
    model::{Event, Message},
    Connection, Discord,
};
use reqwest::header::{self, AUTHORIZATION};

use reqwest::blocking::Client;
use serde_json::to_string_pretty;

use error::AppError;
use openai::{GPTParameters, GPTResponse, OPENAI_URL};

pub struct App {
    openai_key: String,
    discord_token: String,
    discord_api: Option<Discord>,
    discord_conn: Option<Connection>,
    openai_client: Option<Client>,
}

enum IncomingRequest {
    Ignore,
    OpenAIComplete { prompt: String },
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
