pub struct DraftPost {
    content: String
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text)
    }

    pub fn request_review(self) -> PrePendingReview {
        PrePendingReview {
            content: self.content,
        }
    }
}

pub struct Post {
    content: String
}
impl Post {
    pub fn new() -> DraftPost {
        DraftPost { content: String::new() }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct PrePendingReview {
    content: String,
}
impl PrePendingReview {
    pub fn approve(self) -> PendingReview {
        PendingReview { content: self.content }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
}

pub struct PendingReview {
    content: String,
}
impl PendingReview {
    pub fn approve(self) -> Post {
        Post { content: self.content }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost { content: self.content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_published_state() {
        let mut post = Post::new();
        post.add_text("hola que tal las cosas");
        let post = post.request_review();
        let post = post.approve();
        let post = post.approve();
        assert_eq!("hola que tal las cosas", post.content());
    }

    #[test]
    fn test_post_rejected_after_reviewed() {
        let mut post = Post::new();
        post.add_text("hola que tal por alla");
        let post = post.request_review();
        let mut post = post.reject();
        post.add_text(", como esta todo?");
        let post = post.request_review();
        let post = post.approve();
        let post = post.approve();
        assert_eq!("hola que tal por alla, como esta todo?", post.content());
    }
}
