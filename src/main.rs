
mod linkedList;
mod binaryTree;

use crate::linkedList::*;
use crate::binaryTree::*;

fn main() {
    println!("Inserting numbers on the list and printing them.");
    /*let mut tree_root :Node<i32> = Node::new(2);
    println!("Value added to root {}", tree_root.get_value());
    println!("Value added to root {}", tree_root.get_value());*/

    let mut list = LinkedList::new();
    list.insert(42);
    list.insert(43);
    list.insert(4);
    list.insert(5);
    println!("List size is : {}",list.len());
    list.len();
    list.remove(43);    
    println!("List size is : {}",list.len());
    list.len();
    list.remove(4);
    list.print_list();
    println!("List size is : {}",list.len());
    println!("====================================");
    println!("Inserting numbers in the Binary Tree");
    let mut tree = BinaryTree::new();
    tree.insert(10);
    tree.insert(6);
    tree.insert(12);
    tree.insert(12);
    tree.print_tree();

    
}
