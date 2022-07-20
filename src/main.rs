mod app;

use std::{env, fs};

use app::App;

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
