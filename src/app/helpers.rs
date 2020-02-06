use serenity::client::Context;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::{Message, ReactionType};

use json::JsonValue;

use regex::Regex;

use diesel::prelude::*;

use super::post::Post;

use super::reddit::*;

use super::app::AppData;
use super::app::CONFIG;

use super::models;
use super::schema;

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
            e.url(post.post_url())
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

pub fn send_text(ctx: &mut Context, msg: &Message, text: &str) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| m.content(text))?;
    Ok(())
}

pub fn parse_sub(mut args: Args) -> RedditResult<String> {
    let sub_re = Regex::new(r"\b[a-zA-Z0-9]{4,20}\b").unwrap();

    if args.len() > 0 {
        let sub = args.single::<String>().unwrap();
        if !sub_re.is_match(&sub) {
            return Err(RedditAPIError::new("invalid subreddit name"));
        }
        return Ok(sub);
    }

    Ok(CONFIG["default_sub"].to_string())
}

pub fn check_nsfw(ctx: &mut Context, msg: &Message) -> RedditResult<bool> {
    if let Ok(channel) = msg.channel_id.to_channel(ctx) {
        return Ok(channel.is_nsfw());
    }
    Err(RedditAPIError::new("failed to get channel info"))
}

pub fn parse_post(data: &JsonValue) -> RedditResult<Post> {
    let data = &data["data"];

    if data["post_hint"] != "image" && data["post_hint"] != "rich:video" {
        return Err(RedditAPIError::new("post isn't an image"));
    }

    let author = data["author"].to_string();
    let title = data["title"].to_string();
    let permalink = data["permalink"].to_string();
    let mut image = data["url"].to_string();

    let nsfw = match data["over_18"].as_bool() {
        Some(value) => value,
        None => return Err(RedditAPIError::new("failed to get nsfw info for post")),
    };

    if image.contains("gfycat.com") && data["post_hint"] == "rich:video" {
        image = data["secure_media"]["oembed"]["thumbnail_url"].to_string();
    }

    Ok(Post::new(&author, &title, &image, &permalink, nsfw))
}

pub fn get_post(url: &str) -> RedditResult<Post> {
    let res = get_reddit_api(url)?;
    let mut posts = vec![];
    if let JsonValue::Array(arr) = &res["data"]["children"] {
        for post in arr {
            if let Ok(post) = parse_post(post) {
                posts.push(post);
            }
        }
    }
    if posts.len() == 0 {
        return Err(RedditAPIError::new("no posts found"));
    }
    Ok(posts[0].clone())
}

pub fn handle_post(url: &str, ctx: &mut Context, msg: &Message) -> CommandResult {
    match get_post(url) {
        Ok(post) => {
            if post.nsfw && !check_nsfw(ctx, msg)? {
                return send_text(ctx, msg, "this channel isn't nsfw");
            }
            send_post(ctx, msg, &post)
        }
        Err(err) => send_text(ctx, msg, &format!("`{}`", err)),
    }
}

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
        let args = Args::new("sub", &[Delimiter::Single(' ')]);
        parse_sub(args).unwrap();
    }
}
