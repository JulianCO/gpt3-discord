use std::{error::Error, fmt};

#[derive(Debug)]
pub enum AppError {
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
