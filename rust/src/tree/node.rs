#[derive(Debug)]
pub struct Node<T> {
    pub value: T,
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}

impl<T: PartialOrd + Clone> Node<T> {
    pub fn new(value: T) -> Node<T> {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert_left(&mut self, value: T) {
        self.left = Some(Box::new(Node::new(value)));
    }

    pub fn insert_right(&mut self, value: T) {
        self.right = Some(Box::new(Node::new(value)));
    }
}
