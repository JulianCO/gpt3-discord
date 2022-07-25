pub mod config;
pub mod error;
pub mod openai;

use discord::{
    model::{Event, Message},
    Connection, Discord,
};
use reqwest::header::{self, AUTHORIZATION};

use reqwest::blocking::Client;

use config::AppConfig;
use error::AppError;
use openai::{GPTParameters, GPTResponse, OPENAI_URL};

pub struct App {
    config: AppConfig,
    discord_api: Discord,
    discord_conn: Connection,
    openai_client: Client,
}

enum IncomingRequest {
    Ignore,
    OpenAIComplete { prompt: String },
}

impl App {
    pub fn new(app_config: AppConfig) -> Result<Self, AppError> {
        let openai_client = Self::establish_openai_connection(&app_config.openai_key)?;
        let (discord_api, discord_conn) =
            Self::establish_discord_connection(&app_config.discord_token)?;

        Ok(App {
            config: app_config,
            discord_api,
            discord_conn,
            openai_client,
        })
    }

    fn establish_openai_connection(openai_key: &str) -> Result<Client, AppError> {
        let mut default_headers = header::HeaderMap::new();
        let mut auth_header = header::HeaderValue::from_str(&format!("Bearer {}", openai_key))?;
        auth_header.set_sensitive(true);

        default_headers.insert(AUTHORIZATION, auth_header);

        let client = Client::builder().default_headers(default_headers).build()?;

        Ok(client)
    }

    fn establish_discord_connection(
        discord_token: &str,
    ) -> Result<(Discord, Connection), AppError> {
        let discord_api = Discord::from_bot_token(discord_token)?;

        let (discord_conn, _) = discord_api.connect()?;

        Ok((discord_api, discord_conn))
    }

    fn parse_message(&self, message: &Message) -> IncomingRequest {
        let message_text = message.content.as_str();
        if message_text.starts_with(&format!("{} ", self.config.bot_prefix)) {
            IncomingRequest::OpenAIComplete {
                prompt: message_text
                    .split_at(self.config.bot_prefix.len() + 1)
                    .1
                    .to_owned(),
            }
        } else {
            IncomingRequest::Ignore
        }
    }

    pub fn main_loop(&mut self) -> Result<(), AppError> {
        loop {
            match self.discord_conn.recv_event() {
                Ok(Event::MessageCreate(message)) => match self.parse_message(&message) {
                    IncomingRequest::OpenAIComplete { prompt } => {
                        let request_parameters = GPTParameters::new(&prompt, 50);
                        let request = self
                            .openai_client
                            .post(OPENAI_URL)
                            .json(&request_parameters);

                        let res: GPTResponse = request.send()?.json()?;

                        let _ = self.discord_api.send_message(
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
    }
}
