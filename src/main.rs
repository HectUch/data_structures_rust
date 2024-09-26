
mod linked_list;

use crate::linked_list::*;

fn main() {
    println!("Inserting numbers on the list and printing them.");
    /*let mut tree_root :Node<i32> = Node::new(2);
    println!("Value added to root {}", tree_root.get_value());
    println!("Value added to root {}", tree_root.get_value());*/

    let mut my_list = Node_list::new(1);
    my_list.insert(4);
    my_list.insert(7);
    my_list.insert(10);
    my_list.insert(12);
    my_list.insert(-1);
    my_list.print_list();
}
