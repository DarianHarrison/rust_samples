#![allow(warnings)]

// Creating a Tree Data Structure: a Node with Child Nodes

// To start, we’ll build a tree with nodes that know about their child nodes. 
// We’ll create a struct named Node that holds its own i32 value as well as references to its children Node values
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]

// We’ll create a struct named Node that holds its own i32 value as well as references to its children Node values:
// Adding a Reference from a Child to Its Parent
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node { // Creating branch in an inner scope and examining strong and weak reference counts
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //  A leaf node with a weak reference to its parent node branch

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    } // When the inner scope ends, branch goes out of scope and the strong count of the Rc<Node> decreases to 0

    // If we try to access the parent of leaf after the end of the scope, we’ll get None again. 
    // At the end of the program, the Rc<Node> in leaf has a strong count of 1 and a weak count of 0, because the variable leaf is now the only reference to the Rc<Node> again.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
