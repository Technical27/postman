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

fn get_new(sub: &str) -> RedditResult<Post> {
    let res = get_reddit_api(&format!("https://reddit.com/r/{}/new.json", &sub))?;

    if let JsonValue::Array(arr) = &res["data"]["children"] {
        for p in arr.iter() {
            if let Some(post) = parse_post(p) {
                return Ok(post);
            }
        }
    }

    Err(RedditAPIError::new("can't find any image"))
}

#[command]
pub fn new(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let post = get_new(&parse_sub(args)?)?;
    if post.nsfw && !check_nsfw(ctx, msg)? {
        return send_error(ctx, msg, CommandError::boxed("this channel isn't nsfw"));
    }
    send_post(ctx, msg, &post)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_new_posts() {
        let subs = ["dankmemes", "memes", "cursedcomments"];
        let mut passed = false;
        for sub in subs.iter() {
            if let Ok(_) = get_new(sub) {
                passed = true;
                break;
            }
        }
        assert!(passed);
    }
}
