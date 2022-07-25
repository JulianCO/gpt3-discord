pub mod defaults;

use std::{env, fs};

use crate::app::config::AppConfig;

pub fn get_config_from_env() -> AppConfig {
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

    let bot_prefix = match env::var("GPT3_BOT_PREFIX") {
        Ok(s) => s,
        Err(_) => defaults::BOT_PREFIX.to_owned(),
    };

    AppConfig {
        openai_key,
        discord_token,
        bot_prefix,
    }
}
