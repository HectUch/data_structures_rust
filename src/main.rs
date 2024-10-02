
mod linkedList;
mod binaryTree;
mod hstack;

use crate::linkedList::*;
use crate::binaryTree::*;
use crate::hstack::*;

fn test_linked_list(){
    println!("====================================");
    println!("Inserting numbers on the list and printing them.");
    let mut list = LinkedList::new();
    list.insert(42);
    list.insert(43);
    list.insert(4);
    list.insert(5);
    list.print_list();
    println!("Item at(0) {} ",list.at(0));
    println!("Item at(2) {} ",list.at(2));
    println!("List size is : {}",list.len());
    list.len();
    list.remove(43);    
    println!("List size is : {}",list.len());
    list.len();
    list.remove(4);
    list.print_list();
    
    println!("List size is : {}",list.len());
    println!("====================================");
}


fn test_tree(){
    println!("====================================");
    println!("Inserting numbers in the Binary Tree");
    let mut tree = BinaryTree::new();
    tree.insert(10);
    tree.insert(7);
    tree.insert(8);
    tree.insert(15);
    tree.insert(18);
    tree.insert(16);
    
    tree.print_tree();    
    tree.remove(10);
    tree.print_tree();
    println!("====================================");
}

fn test_stack(){
    println!("====================================");
    println!("Testing Stack");
    let mut test_stack = my_Stack::new();
    test_stack.push(2);
    test_stack.push(8);
    test_stack.push(9);
    test_stack.print_top();
    test_stack.push(10);
    test_stack.print_top();
    println!("Popping {} ",test_stack.pop());
    println!("Popping {} ",test_stack.pop());
    test_stack.print_top();
    println!("====================================");
}

fn main() {
    
    test_linked_list();
    test_tree();
    test_stack();

    
}
