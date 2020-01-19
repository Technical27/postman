use json::{self, JsonValue};
use reqwest::blocking as reqwest;

use super::helpers::*;

use std::error;

#[derive(Debug, Clone)]
pub struct RedditAPIError {
    msg: String,
}

impl RedditAPIError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }
}

impl std::fmt::Display for RedditAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.msg)?;
        Ok(())
    }
}

impl error::Error for RedditAPIError {}

impl From<::reqwest::Error> for RedditAPIError {
    fn from(error: ::reqwest::Error) -> Self {
        Self {
            msg: format!("http error: {}", error),
        }
    }
}

impl From<json::JsonError> for RedditAPIError {
    fn from(error: json::JsonError) -> Self {
        Self {
            msg: format!("json parsing error: {}", error),
        }
    }
}

pub fn get_reddit_api(url: &str) -> Result<JsonValue, RedditAPIError> {
    let http = reqwest::Client::builder()
        .user_agent(load_data()["user_agent"].to_string())
        .build()?;

    let text = http.get(url).send()?.text()?;

    Ok(json::parse(&text)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reddit_api() {
        get_reddit_api("https://reddit.com/r/memes/random.json").unwrap();
    }
}
