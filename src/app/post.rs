pub struct Post {
    pub author: String,
    pub title: String,
    pub image: String,
    pub permalink: String,
    pub nsfw: bool
}

impl Post {
    pub fn new(author: String, title: String, image: String, permalink: String, nsfw: bool) -> Self {
        Self { author, title, image, permalink, nsfw }
    }
    pub fn author_url (&self) -> String {
        format!("https://reddit.com/u/{}", &self.author)
    }
    pub fn post_url (&self) -> String {
       format!("https://reddit.com{}", &self.permalink)
    }
}
