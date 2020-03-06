use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::{Message, ReactionType};

use std::env;

use json::JsonValue;

use regex::Regex;

use log::trace;

use lazy_static::lazy_static;

use diesel::prelude::*;

use super::post::*;

use super::reddit::*;

use super::app::AppData;

use super::models;
use super::schema;

// helper method to send a post on discord
pub fn send_post(ctx: &mut Context, msg: &Message, post: &Post) -> CommandResult {
    use schema::messages::dsl::*;

    let sent_msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(&post.title);
            e.author(|a| {
                a.name(&post.author);
                a.url(post.author_url())
            });
            e.image(&post.image);
            e.url(post.post_url());
            e.footer(|f| f.text(format!("{} upvotes", post.ups)))
        })
    })?;

    let mut client_data = ctx.data.write();
    let appdata = client_data.get_mut::<AppData>().unwrap();
    let conn = appdata.db_pool.get().unwrap();

    diesel::insert_into(messages)
        .values(&models::Message::new(&sent_msg, msg))
        .execute(&conn)
        .expect("failed saving message");

    sent_msg.react(&ctx, ReactionType::Unicode("\u{274C}".to_string()))?;

    Ok(())
}

// helper method to send a pure text message on discord
pub fn send_text(ctx: &Context, msg: &Message, text: &str) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| m.content(text))?;
    Ok(())
}

lazy_static! {
    static ref SUB_REGEX: Regex = { Regex::new(r"\b[a-zA-Z0-9]{3,20}\b").unwrap() };
}

#[derive(Debug, Clone)]
pub enum ParseSubError {
    NoDefaultSub,
    InvalidSub,
}

impl std::fmt::Display for ParseSubError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoDefaultSub => write!(
                f,
                "```POSTMAN_DEFUALT_SUB wasn't specified, contact the bot hoster/admin```"
            ),
            Self::InvalidSub => write!(f, "```that sub can't exist, try a different one```"),
        }
    }
}

// parses command arguments and returns a subreddit name
pub fn parse_sub(mut args: Args) -> Result<String, ParseSubError> {
    let default_sub = if let Ok(sub) = env::var("POSTMAN_DEFAULT_SUB") {
        sub
    } else {
        return Err(ParseSubError::NoDefaultSub);
    };

    let sub = args.single::<String>().unwrap_or(default_sub);

    if !SUB_REGEX.is_match(&sub) {
        Err(ParseSubError::InvalidSub)
    } else {
        Ok(sub)
    }
}

// helper to check if a discord channel is nsfw
pub fn check_nsfw(ctx: &mut Context, msg: &Message) -> RedditResult<bool> {
    match msg.channel_id.to_channel(ctx) {
        Ok(channel) => Ok(channel.is_nsfw()),
        Err(_) => Err(RedditAPIError::new("failed to get channel info")),
    }
}

// turns raw json into a Post object
pub fn parse_post(data: &JsonValue) -> Option<Post> {
    let data = &data["data"];

    if data["post_hint"] != "image" && data["post_hint"] != "rich:video" {
        return None;
    }

    let author = data["author"].to_string();
    let title = data["title"].to_string();
    let permalink = data["permalink"].to_string();

    let nsfw = data["over_18"].as_bool().unwrap();

    let image = if data["url"].contains("gfycat.com") && data["post_hint"] == "rich:video" {
        data["secure_media"]["oembed"]["thumbnail_url"].to_string()
    } else {
        data["url"].to_string()
    };

    let ups = data["ups"].as_u64().unwrap();

    Some(Post::new(&author, &title, &image, &permalink, nsfw, ups))
}

// method to get a post on reddit from a list endpoint
pub fn get_post(url: &str, nsfw: bool) -> PostResult {
    let res = get_reddit_api(url)?;
    let mut posts = vec![];

    if let JsonValue::Array(arr) = &res["data"]["children"] {
        posts = arr.into_iter().map(|x| parse_post(x)).collect();
    }

    if posts.len() == 0 {
        return Err(PostError::NoPostsFound);
    }

    let posts: Vec<Post> = posts
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();

    if posts.len() == 0 {
        return Err(PostError::NoImagesFound);
    }

    match posts.into_iter().find(|x| !x.nsfw || nsfw) {
        Some(post) => {
            trace!("sending post: {:?}", post);
            Ok(post)
        }
        None => Err(PostError::NoSafePostsFound),
    }
}

// gets and sends a post
pub fn handle_post(
    fmt: &'static str,
    args: Args,
    ctx: &mut Context,
    msg: &Message,
) -> CommandResult {
    match parse_sub(args) {
        Ok(sub) => {
            let url = &format!("https://reddit.com/r/{}/{}", sub, fmt);
            match get_post(url, check_nsfw(ctx, msg)?) {
                Ok(post) => send_post(ctx, msg, &post),
                Err(e) => {
                    trace!("error while getting post: {}", e);
                    send_text(ctx, msg, &format!("{}", e))
                }
            }
        }
        Err(e) => send_text(ctx, msg, &format!("{}", e)),
    }
}

// convenience for getting the current version
pub fn get_version() -> String {
    format!(
        "{}.{}.{}",
        env!("CARGO_PKG_VERSION_MAJOR"),
        env!("CARGO_PKG_VERSION_MINOR"),
        env!("CARGO_PKG_VERSION_PATCH")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use serenity::framework::standard::{Args, Delimiter};

    #[test]
    fn can_parse_valid_sub() {
        let args = Args::new("memes", &[Delimiter::Single(' ')]);
        parse_sub(args).unwrap();
    }

    #[test]
    #[should_panic]
    fn sub_with_invalid_name() {
        let args = Args::new("&", &[Delimiter::Single(' ')]);
        parse_sub(args).unwrap();
    }

    #[test]
    #[should_panic]
    fn sub_with_a_long_name() {
        let args = Args::new("reallylongsubnamethatcantexist", &[Delimiter::Single(' ')]);
        parse_sub(args).unwrap();
    }

    #[test]
    #[should_panic]
    fn sub_with_a_short_name() {
        let args = Args::new("su", &[Delimiter::Single(' ')]);
        parse_sub(args).unwrap();
    }
}
