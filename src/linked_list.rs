pub struct Node_list<i32> {
    value: i32,
    next_node: Option<Box<Node_list<i32>>>,
}

impl <i32 : std::fmt::Debug> Node_list<i32>{

    pub fn new(new_value:i32) -> Self{
        Self{ value : new_value, next_node : None}
    }

    pub fn insert(&mut self, value :i32){
        if (!self.next_node.is_none()){
            if let Some(next_node_list) = &mut self.next_node{
                next_node_list.insert(value);
            }
        }
        else{
            self.next_node = Some(Box::new(Node_list::new(value)))
        }
    }

    pub fn print_list(&self){
        
        print!(" {:?} ->", &self.value);      
       

        if let Some(next_node) = &self.next_node{
            next_node.print_list();
        }
        else{
            println!(" null ");
        }
    }
    
}
