use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
};

use super::helpers::*;
use super::post::Post;
use super::reddit::*;
use super::CommandError;
use json::JsonValue;

fn get_rising(sub: &str) -> RedditResult<Post> {
    let res = get_reddit_api(&format!("https://reddit.com/r/{}/rising.json", &sub))?;

    if let JsonValue::Array(arr) = &res["data"]["children"] {
        for p in arr {
            if let Some(post) = parse_post(p) {
                return Ok(post);
            }
        }
    }

    Err(RedditAPIError::new("can't find any image"))
}

#[command]
pub fn rising(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let post = get_rising(&parse_sub(args)?)?;
    if post.nsfw && !check_nsfw(ctx, msg)? {
        return send_error(ctx, msg, CommandError::boxed("this channel isn't nsfw"));
    }
    send_post(ctx, msg, &post)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_rising_posts() {
        let subs = ["dankmemes", "memes", "cursedcomments"];
        let mut passed = false;
        for sub in subs.iter() {
            if let Ok(_) = get_rising(sub) {
                passed = true;
                break;
            }
        }
        assert!(passed);
    }
}
