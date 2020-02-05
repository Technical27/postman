#[derive(Debug, Clone)]
pub struct Post {
    pub author: String,
    pub title: String,
    pub image: String,
    pub permalink: String,
    pub nsfw: bool,
}

impl Post {
    pub fn new(author: &str, title: &str, image: &str, permalink: &str, nsfw: bool) -> Self {
        Self {
            author: author.to_string(),
            title: title.to_string(),
            image: image.to_string(),
            permalink: permalink.to_string(),
            nsfw,
        }
    }
    pub fn author_url(&self) -> String {
        format!("https://reddit.com/u/{}", &self.author)
    }
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

        let post = Post::new(author, title, image, permalink, false);

        assert!(!post.nsfw);

        assert_eq!(post.author, author);
        assert_eq!(post.title, title);
        assert_eq!(post.image, image);
        assert_eq!(post.permalink, permalink);
    }

    #[test]
    fn post_author_url() {
        let post = Post::new("Technical27", "title", "image_url", "/permalink", false);

        assert_eq!(
            post.author_url(),
            "https://reddit.com/u/Technical27".to_string()
        );
    }

    #[test]
    fn post_permalink() {
        let post = Post::new("Technical27", "title", "image_url", "/permalink", false);

        assert_eq!(post.post_url(), "https://reddit.com/permalink");
    }
}
