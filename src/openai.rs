use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GPTParameters {
    pub model: String,
    pub prompt: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GPTResponse {
    choices: Vec<GPTChoice>,
    usage: GPTUsageStats,
}

#[derive(Serialize, Deserialize)]
pub struct GPTChoice {
    text: String,
    index: u32,
    finish_reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct GPTUsageStats {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}