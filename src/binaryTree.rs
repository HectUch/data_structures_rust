struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {    
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

pub struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {

    pub fn new() -> Self{
        BinaryTree { root : None }
    }

    pub fn insert(&mut self,_value : i32) -> bool {

        if self.root.is_none(){
            self.root = Some(Box::new(Node::new(_value)));
            println!("Value added.")
        }
        else {
            let mut current = &mut self.root;
            while let Some(ref mut node) = current {
                if _value < node.value {
                    if node.left.is_none() {
                        node.left = Some(Box::new(Node::new(_value)));
                        println!("Value added.");
                        break;
                    }
                    else{
                        current = &mut node.left;
                        continue;
                    } 
                }
                else if _value > node.value {
                    if node.right.is_none() {
                        node.right = Some(Box::new(Node::new(_value)));
                        println!("Value added.");
                        break;
                    }
                    else{
                        current = &mut node.right;
                        continue;
                    } 
                }
                else {
                    println!("Value already in.");
                    return false;
                }
                  
            }
        }

        true
    }

    fn easy_recursive(leaf : Node){
        print!(" {} -",leaf.value);
        if let Some(ref right) = &leaf.right{
            Self::easy_recursive(right);
        }
        if let Some(ref left) = &leaf.left{
            Self::easy_recursive(left);
        }

    }

    pub fn print_tree(&self){
        
        if !self.root.is_none(){
            println!("something");           
            return;
        }

        println!("Empty tree");
    }

}