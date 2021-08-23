// Definition of a Post struct and a new function that creates a new Post instance, a State trait, and a Draft struct

// Let’s get started on the implementation of the library!

// We know we need a public Post struct that holds some content, 
// so we’ll start with the definition of the struct and an associated public new function to create an instance of Post

// We’ll also make a private State trait. 
// Then Post will hold a trait object of Box<dyn State> inside an Option<T> in a private field named state. 





pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    // Implementing the add_text method to add text to a post’s content.
    pub fn add_text(&mut self, text: &str) { // The add_text method takes a mutable reference to self, because we’re changing the Post instance that we’re calling add_text on. 
        self.content.push_str(text);
    }
}

trait State {} // The State trait defines the behavior shared by different post states, and the Draft, PendingReview, and Published states will all implement the State trait. 

struct Draft {}

impl State for Draft {}