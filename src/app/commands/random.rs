use serenity::{
    framework::standard::{
        CommandResult,
        Args,
        macros::command
    },
    client::Context,
    model::prelude::Message
};

use json::{self, JsonValue};

use super::helpers::*;
use super::reddit::*;
use super::post::Post;
use super::CommandError;

fn get_random (sub: &str) -> Result<Post, RedditAPIError> {
    let mut i = 0;

    let post =
    loop {
        if i == 5 { return Err(RedditAPIError::new("cant find any image")); }
        let res = get_reddit_api(&format!("https://reddit.com/r/{}/random.json", sub))?;
        if let JsonValue::Object(p) = &res[0]["data"]["children"][0] {
            let data = &p["data"];
            if data["post_hint"] != "image" {
                i += 1;
                continue;
            }
            let author = &data["author"].to_string();
            let title = &data["title"].to_string();
            let image = &data["url"].to_string();
            let permalink = &data["permalink"].to_string();
            let nsfw = data["over_18"].as_bool().unwrap();
            break Post::new(author, title, image, permalink, nsfw);
        }
        i += 1;
    };

    Ok(post)
}

#[command]
pub fn random(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let post = get_random(&parse_sub(args)?)?;
    if post.nsfw && !check_nsfw(ctx, msg)? {
        return send_error(ctx, msg, CommandError::boxed("this channel isn't nsfw"));
    }
    send_post(ctx, msg, &post)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_random_posts() {
        let subs = ["dankmemes", "memes", "cursedcomments"];
        let mut passed = false;
        for sub in subs.iter() {
            if let Ok(_) = get_random(sub) {
                passed = true;
                break;
            }
        }
        assert!(passed);
    }
    #[test]
    fn posts_are_random() {
        let post1 = get_random("memes").unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
        let post2 = get_random("memes").unwrap();
        let equal =
            post1.title == post2.title &&
            post1.author == post2.author;

        assert!(!equal);
    }
}
