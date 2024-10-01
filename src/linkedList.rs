struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {    
    fn new(value: i32) -> Self {
        Node {
            value,
            next: None,
        }
    }
}

pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    
    pub fn new() -> Self {
        LinkedList { head: None }
    }
    
    pub fn insert(&mut self, _value: i32) {

        if self.head.is_none(){
            self.head = Some(Box::new(Node::new(_value)));
            println!("Value added.")
        }
        else {
            let mut current = &mut self.head;
            while let Some(ref mut node) = current {
                if node.next.is_none() {
                    node.next = Some(Box::new(Node::new(_value)));
                    println!("Value added.");
                    return;
                } 
                current = &mut node.next;  
            }
        }
    }

    pub fn remove(&mut self, _value: i32) -> bool { 
        println!("Removing {} ...", _value);       
        let mut current = &mut self.head;

        if current.is_none() {
            return false;  // List is empty
        }    
        
        if let Some(ref mut node) = current {
            if node.value == _value {
                self.head = node.next.take();  
                return true;
            }
        }    
        
        while let Some(ref mut node) = current {
            if let Some(ref mut next_node) = node.next {
                if next_node.value == _value {
                    node.next = next_node.next.take();  
                    return true;
                }
            }
            current = &mut node.next;  
        }
    
        false 
    }

    pub fn at(&mut self,_index :i32) -> i32{
        let mut index = 0;
        let mut current = &mut self.head;

        if current.is_none() {
            panic!("Out of bounds");  // List is empty
        }      
        
        while let Some(ref mut node) = current {
            if index == _index {
                return node.value;               
            }
            current = &mut node.next;
            index += 1;  
        }    
        panic!("Out of bounds"); 

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

