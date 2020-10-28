use std::{fmt, mem};
use std::fmt::Formatter;

///    ## Things To Note:
///    - Traits define what something can do.
//     - Think of a Java class's functions declaration
//     - Impls are implementations, think of writing what the function *does* in Java
//     - A struct is a record - no more than a fixed collection of values called fields
//     - & means you are passing it by reference and therefore not moving/copying
//     - * is dereferencing
//     - mut (mutable) means you can write to the memory; so its modifiable
//     - if a struct is not mutable you also cannot write to any of its members

// Structure for a node
struct TreeNode<'a> {
    value: &'a i32, // A i32 for value of the node
    left: Option<Box<TreeNode<'a>>>, // A boxed node on the left side (the size can be infinite so we need to box it)
    right: Option<Box<TreeNode<'a>>>, // A boxed node for the right side
}

// Implementation of TreeNode
impl<'a> TreeNode<'a> {

    //Inserts a node on specified branch with specified value
    pub fn insert(&mut self, new_value: &'a i32, insert_left_branch: bool) {
        //If insert_left_branch is true, node is self.left; otherwise its the right node
        let node = if insert_left_branch {&mut self.left} else {&mut self.right};
        //Create an empty node
        let new_node = TreeNode{value: new_value, left: None, right: None};
        //Box our node
        let boxxed_node= Box::new(new_node);
        //Set node as a dereferenced boxxed_node
        *node = Option::from(boxxed_node);
    }

    //Function to set a node's value
    pub fn set_value(&mut self, new_value: &'a i32) {
        self.value = new_value;
    }

    //Invert the node and all its children
    pub fn invert(&mut self) {
        //If both nodes are None, return;
        if self.left.is_none() && self.right.is_none() {return;};
        //Swap, in memory, the left and right nodes (One of my favourite things about rust)
        mem::swap(&mut self.right, &mut self.left);
        // If right is something, invert it
        if let Some(right) = &mut self.right { right.invert(); }
        // If left is something, invert it
        if let Some(left) = &mut self.left { left.invert(); }
    }
}

//Implementation of fmt::Display for TreeNode
impl fmt::Display for TreeNode<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        //format! macro
        write!(f, "{}", format!(
            "{{\"v\": {}, \"l\": {}, \"r\": {}}}", //Text before formatting
            self.value, // First value is the value of the node
            match &self.left { // Match self.left
                Some(left) => format!("{}", left), // If it is something its itself formatted
                _ => String::from("{}") // If not it is {}
            },
            match &self.right {
                Some(right) => format!("{}", right),
                _ => String::from("{}")
            }
        ))
    }
}

//Finally lets put our nodes to use
fn main() {
    let mut binary_tree = TreeNode{value: &0, left: None, right: None}; // Create our binary tree
    binary_tree.set_value(&1); //Set the value of the root root of the tree to 1
    binary_tree.insert(&2, false); // Create a branch with a value of 2 on our right side
    binary_tree.insert(&3, true); // Create a branch with a value of 3 on our left side
    binary_tree.invert(); // Invert our Binary Tree
    println!("Binary Tree:\n{}", binary_tree);
}

