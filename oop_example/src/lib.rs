// define a trait named Draw with one method named draw:
pub trait Draw {
    fn draw(&self); // 
}

pub struct Screen { // a struct named Screen that holds a vector named components. 
    pub components: Vec<Box<dyn Draw>>, // This vector is of type Box<dyn Draw>, which is a trait object; it's a stand-in for any type inside a Box that implements the Draw trait.
}

  
// On the Screen struct, we’ll define a method named run that will call the draw method on each of its components
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// A Button struct that implements the Draw trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

// The Self keyword is an alias for the type we’re implementing the traits or methods on.
