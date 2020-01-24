use serenity::{
    client::Context,
    framework::standard::{Args, CommandResult},
    model::prelude::Message,
    model::prelude::ReactionType,
};

use json::JsonValue;

use regex::Regex;

use std::{error, fs};

use super::post::Post;
use super::reddit::RedditAPIError;
use diesel::prelude::*;

use super::app::AppData;

use super::models;
use super::schema;

pub fn load_data() -> JsonValue {
    let data = json::parse(
        &String::from_utf8(fs::read("config.json").expect("failed to load config file")).unwrap(),
    )
    .expect("failed to parse config file");
    data
}

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

pub fn send_error(ctx: &mut Context, msg: &Message, err: Box<dyn error::Error>) -> CommandResult {
    msg.channel_id
        .send_message(&ctx.http, |m| m.content(format!("`{}`", err)))?;
    println!("error while running command: {}", err);
    Ok(())
}

pub fn parse_sub(mut args: Args) -> Result<String, RedditAPIError> {
    let sub_re = Regex::new(r"\b[a-zA-Z0-9]{4,20}\b").unwrap();

    if args.len() > 0 {
        let sub = args.single::<String>().unwrap();
        if !sub_re.is_match(&sub) {
            return Err(RedditAPIError::new("invalid subreddit name"));
        }
        return Ok(sub);
    }

    Ok(load_data()["default_sub"].to_string())
}

pub fn check_nsfw(ctx: &mut Context, msg: &Message) -> Result<bool, &'static str> {
    if let Ok(channel) = msg.channel_id.to_channel(ctx) {
        return Ok(channel.is_nsfw());
    }
    Err("failed to check channel")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serenity::framework::standard::{Args, Delimiter};

    #[test]
    fn loads_data_from_config() {
        load_data();
    }

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
