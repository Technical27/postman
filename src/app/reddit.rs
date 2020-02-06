use json::JsonValue;
use reqwest::blocking;

use super::helpers::get_version;

use lazy_static::lazy_static;

use log::trace;

use std::error;

#[derive(Debug, Clone)]
pub struct RedditAPIError(String);

impl RedditAPIError {
    pub fn new(msg: &str) -> Self {
        Self(msg.to_string())
    }
}

pub type RedditResult<T> = Result<T, RedditAPIError>;

impl std::fmt::Display for RedditAPIError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

impl error::Error for RedditAPIError {}

impl From<reqwest::Error> for RedditAPIError {
    fn from(error: reqwest::Error) -> Self {
        Self(format!("http error: {}", error))
    }
}

impl From<json::JsonError> for RedditAPIError {
    fn from(error: json::JsonError) -> Self {
        Self(format!("json parsing error: {}", error))
    }
}

lazy_static! {
    static ref USER_AGENT: String = format!("postman v{} by /u/Technical27", get_version());
}

pub fn get_reddit_api(url: &str) -> RedditResult<JsonValue> {
    trace!("requesting url {}", url);
    let http = blocking::Client::builder()
        .user_agent(USER_AGENT.as_str())
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
