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
    pub choices: Vec<GPTChoice>,
    pub usage: GPTUsageStats,
}

#[derive(Serialize, Deserialize)]
pub struct GPTChoice {
    pub text: String,
    pub index: u32,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct GPTUsageStats {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}