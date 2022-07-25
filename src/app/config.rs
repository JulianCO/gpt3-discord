#[derive(Debug, Clone)]
pub struct AppConfig {
    pub openai_key: String,
    pub discord_token: String,
    pub bot_prefix: String,
}
