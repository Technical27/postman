use super::reddit::RedditAPIError;

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
}

impl Post {
    pub fn new(
        author: &str,
        title: &str,
        image: &str,
        permalink: &str,
        nsfw: bool,
        ups: u64,
    ) -> Self {
        Self {
            author: author.to_string(),
            title: title.to_string(),
            image: image.to_string(),
            permalink: permalink.to_string(),
            nsfw,
            ups,
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

// an enum to represent the different error that can happen when parsing posts
#[derive(Debug, Clone)]
pub enum PostError {
    // the sub doesn't contain any posts
    NoPostsFound,
    // can't find any images in the sub
    NoImagesFound,
    // can't find any sfw images in the sub
    NoSafePostsFound,
    // an error with the api/json parsing
    RedditError(RedditAPIError),
}

pub type PostResult = Result<Post, PostError>;

impl std::fmt::Display for PostError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoPostsFound => write!(f, "```can't find any posts in that sub```"),
            Self::NoImagesFound => write!(f, "```can't find any images```"),
            Self::NoSafePostsFound => write!(f, "```no safe-for-work posts found```"),
            Self::RedditError(e) => write!(f, "```something broke:\n{:?}```", e),
        }
    }
}

impl std::error::Error for PostError {}

impl From<RedditAPIError> for PostError {
    fn from(e: RedditAPIError) -> Self {
        PostError::RedditError(e)
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

        let post = Post::new(author, title, image, permalink, false, 69);

        assert!(!post.nsfw);

        assert_eq!(post.author, author);
        assert_eq!(post.title, title);
        assert_eq!(post.image, image);
        assert_eq!(post.permalink, permalink);
        assert_eq!(post.ups, 69);
    }

    #[test]
    fn post_author_url() {
        let post = Post::new("Technical27", "title", "image_url", "/permalink", false, 0);

        assert_eq!(
            post.author_url(),
            "https://reddit.com/u/Technical27".to_string()
        );
    }

    #[test]
    fn post_permalink() {
        let post = Post::new("Technical27", "title", "image_url", "/permalink", false, 0);

        assert_eq!(post.post_url(), "https://reddit.com/permalink");
    }
}
