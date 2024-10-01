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

    pub fn remove(&mut self, value: i32) -> bool {
        self.root = Self::remove_node(self.root.take(), value);
        self.root.is_some() 
    }

    fn remove_node(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        let mut node = node?;
        
        if value < node.value {
            node.left = Self::remove_node(node.left, value);
            Some(node)
        } else if value > node.value {
            node.right = Self::remove_node(node.right, value);
            Some(node)
        } else {
            
            if node.left.is_none() {
                return node.right; 
            } else if node.right.is_none() {
                return node.left; 
            } else {
                let min_node = Self::find_min(&mut node.right);
                node.value = min_node.value; 
                node.right = Self::remove_node(node.right, min_node.value); 
                Some(node)
            }
        }
    }
    
    fn find_min(node: &mut Option<Box<Node>>) -> Box<Node> {
        let mut current = node.as_mut().unwrap();
        while let Some(ref mut left) = current.left {
            current = left;
        }        
        Box::new(Node::new(current.value))
    }
    //Try improving this with ?
    fn easy_recursive(leaf : &Node){
        print!(" {} ",leaf.value);
        if let Some(ref right) = &leaf.right{
            print!(" - ");
            Self::easy_recursive(right);
        }
        if let Some(ref left) = &leaf.left{
            print!(" - ");
            Self::easy_recursive(left);
        }
    }

    pub fn print_tree(&self){        
        if !self.root.is_none(){
            if let Some(ref root_leaf) = &self.root{
            Self::easy_recursive(root_leaf);  
            println!("");       
            return;
            }
        }
        println!("Empty tree");
    }

}