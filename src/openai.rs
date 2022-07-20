use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GPTParameters {
    pub model: String,
    pub prompt: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

impl GPTParameters {
    pub fn new(prompt: &str, max_tokens: u32) -> Self {
        GPTParameters {
            model: "text-davinci-002".to_string(),
            prompt: prompt.to_owned(),
            temperature: 0.7,
            max_tokens,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GPTResponse {
    pub choices: Vec<GPTChoice>,
    pub usage: GPTUsageStats,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GPTChoice {
    pub text: String,
    pub index: u32,
    pub finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GPTUsageStats {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
