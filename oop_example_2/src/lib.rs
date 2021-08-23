// Definition of a Post struct and a new function that creates a new Post instance, a State trait, and a Draft struct

// Let’s get started on the implementation of the library!

// We know we need a public Post struct that holds some content, 
// so we’ll start with the definition of the struct and an associated public new function to create an instance of Post
// We’ll also make a private State trait. 
// Then Post will hold a trait object of Box<dyn State> inside an Option<T> in a private field named state. 

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    // --snip--
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
