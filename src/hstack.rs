
use crate::linkedList::*;

pub struct my_Stack {
    pile : LinkedList,
    top : i32,
}

impl my_Stack {
    pub fn new() -> Self {
        my_Stack {
            pile: LinkedList::new(),
            top : -1,
        }
    }
    
    pub fn push(&mut self, _value: i32) {       
        self.pile.insert(_value);
        self.top += 1;
        println!("Value added.");        
    }

    pub fn pop(&mut self) -> i32 { 
        println!("pop ...");       
        let popped_value = self.pile.at(self.top);
        self.top -= 1;
        self.pile.remove(popped_value);
        popped_value       
    }

    pub fn print_top(&mut self){
        println!("Item at the top: {}",self.pile.at(self.top));
    }

    pub fn len(&mut self) -> u32{
        self.pile.len()        
    }
}