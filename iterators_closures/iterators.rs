


// Processing a Series of Items with Iterators

// The Iterator Trait and the next Method


// calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. 
// In other words, this code consumes, or uses up, the iterator. 
// Each call to next eats up an item from the iterator

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
