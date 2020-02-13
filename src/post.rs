// a struct to represent a post on reddit
#[derive(Debug, Clone)]
pub struct Post {
    pub author: String,
    pub title: String,
    pub image: String,
    // an incomplete link to the post
    pub permalink: String,
    pub nsfw: bool,
    pub ups: u64,
    pub downs: u64
}

impl Post {
    pub fn new(author: &str, title: &str, image: &str, permalink: &str, nsfw: bool, ups: u64, downs: u64) -> Self {
        Self {
            author: author.to_string(),
            title: title.to_string(),
            image: image.to_string(),
            permalink: permalink.to_string(),
            nsfw,
            ups,
            downs,
        }
    }
    // returns a link to the author of the post
    pub fn author_url(&self) -> String {
        format!("https://reddit.com/u/{}", &self.author)
    }
    // returns a link to the post itself
    pub fn post_url(&self) -> String {
        format!("https://reddit.com{}", &self.permalink)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn post_constructor() {
        let author = "Technical27";
        let title = "Some random Post";
        let image = "https://some_random_url.com";
        let permalink = "/r/memes";

        let post = Post::new(author, title, image, permalink, false, 69, 420);

        assert!(!post.nsfw);

        assert_eq!(post.author, author);
        assert_eq!(post.title, title);
        assert_eq!(post.image, image);
        assert_eq!(post.permalink, permalink);
        assert_eq!(post.ups, 69);
        assert_eq!(post.downs, 420);
    }

    #[test]
    fn post_author_url() {
        let post = Post::new("Technical27", "title", "image_url", "/permalink", false, 0, 0);

        assert_eq!(
            post.author_url(),
            "https://reddit.com/u/Technical27".to_string()
        );
    }

    #[test]
    fn post_permalink() {
        let post = Post::new("Technical27", "title", "image_url", "/permalink", false, 0, 0);

        assert_eq!(post.post_url(), "https://reddit.com/permalink");
    }
}
