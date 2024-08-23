use std::collections::LinkedList;

use super::node::Node;

fn bfs<T>(root: Option<Box<Node<T>>>) -> Vec<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    let mut result: Vec<T> = Vec::new();

    let mut queue: LinkedList<&Box<Node<T>>> = LinkedList::new();
    let binding = root.unwrap();
    let mut current: Option<&Box<Node<T>>> = Some(&binding);
    loop {
        match &current {
            Some(node) => {
                let left = &node.left;
                let right = &node.right;

                result.push(node.value.clone());

                if let Some(node) = left {
                    queue.push_front(node);
                }
                if let Some(node) = right {
                    queue.push_front(node);
                }
            }
            None => {}
        }

        if queue.len() == 0 {
            break;
        }

        current = Some(queue.pop_back().unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::tree::{bfs::bfs, node::Node};

    #[test]
    fn basic() {
        let mut root = Box::new(Node::new(5));
        root.insert_left(3);
        root.insert_right(7);

        if let Some(node) = &mut root.left {
            node.insert_left(2);
            node.insert_right(4);
        }

        if let Some(node) = &mut root.right {
            node.insert_right(8);
            node.insert_left(6);
        }

        let result = bfs(Some(root));

        assert_eq!(result, [5, 3, 7, 2, 4, 6, 8]);
    }
}
