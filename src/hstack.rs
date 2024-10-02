
use crate::linkedList::*;

pub struct  my_Stack<T> {
    pile : LinkedList<T>,
    top : usize,
}

impl <T: PartialEq + std::fmt::Debug + Clone> my_Stack<T> {
    pub fn new() -> Self {
        my_Stack {
            pile: LinkedList::new(),
            top : 0,
        }
    }
    
    pub fn push(&mut self, _value : T) {       
        self.pile.insert(_value);
        self.top += 1;
        println!("Value added.");        
    }

    pub fn pop(&mut self) -> T {
        if self.top == 0 {
            panic!("Stack underflow");
        }             
        let popped_value = self.pile.at(self.top-1).clone();
        self.top -= 1;
        self.pile.remove(popped_value.clone());
        popped_value       
    }

    pub fn print_top(&mut self){
        if self.top == 0 {
            println!("Stack is empty");
        } else {
            println!("Item at the top: {:?}", self.pile.at((self.top-1)));
        }
    }

    pub fn len(&mut self) -> u32{
        self.pile.len()        
    }
}