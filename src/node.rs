
/// A node in the binary tree.
#[derive(Debug)]
pub struct Node<i32> {
    value: i32,
    left: Option<Box<Node<i32>>>,
    right: Option<Box<Node<i32>>>,
}

impl <i32> Node<i32>{
    pub fn new(item : i32) -> Self {
        Self { value : item, left : None, right :None }
    
    }
    pub fn get_value(&self) -> &i32{
        &self.value
    }

    pub fn insert(&self,value:i32) -> bool{
        if Some(value) > self.value && self.right.is_none(){
            self.right = Some(Box::new(Node::new(value)));
            true
        }
        else if value < self.value && self.right.is_none() {
            true
        }
        else {
            false
        }

    }

   /* fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }*/

}