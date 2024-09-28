
mod linked_list;

use crate::linked_list::*;

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

    
}
