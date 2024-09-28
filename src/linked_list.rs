struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    // Constructor to create a new Node
    fn new(value: i32) -> Self {
        Node {
            value,
            next: None,
        }
    }
}
// Define a LinkedList struct
pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    // Constructor to create a new LinkedList
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    // Function to add a node at the beginning of the list
    pub fn insert(&mut self, _value: i32) {

        if self.head.is_none(){
            self.head = Some(Box::new(Node::new(_value)));
            println!("Value added.")
        }
        else {
            let mut current = &mut self.head;  // Get a mutable reference to head

            while let Some(ref mut node) = current {
                if node.next.is_none() {
                    node.next = Some(Box::new(Node::new(_value)));
                    println!("Value added.");
                    return;
                } 
                current = &mut node.next;  // Move to the next node
            }
        }
    }

    pub fn remove(&mut self, _value: i32) -> bool { 
        println!("Removing {} ...", _value);       
        let mut current = &mut self.head;  // Get mutable reference to head

        if current.is_none() {
            return false;  // List is empty
        }
    
        // Handle removing the head of the list if it matches the value
        if let Some(ref mut node) = current {
            if node.value == _value {
                self.head = node.next.take();  // Remove the head by pointing it to the next node
                return true;
            }
        }
    
        // Traverse the list to find the node to remove
        while let Some(ref mut node) = current {
            if let Some(ref mut next_node) = node.next {
                if next_node.value == _value {
                    node.next = next_node.next.take();  // Remove the node by skipping it
                    return true;
                }
            }
            current = &mut node.next;  // Move to the next node
        }
    
        false // Node not found
    }

    pub fn print_list(&mut self){

        let mut current = &mut self.head;

        while let Some(ref mut node) = current {
            print!(" {:?} ->", node.value);
            current = &mut node.next;      
        }
        
        println!(" null ");
      
    }

    pub fn len(&mut self) -> u32{
        let mut size = 0;
        

        if !self.head.is_none() {
            let mut current = &mut self.head;
            while let Some(ref mut node) = current {
                size += 1;
                current = &mut node.next;  
            }
        }

        size
    }
}

