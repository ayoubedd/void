use super::node::Node;

// problem: Determine if a path exists from the root of the tree to a leaf node.
// it may not ctain any zeros

fn backtracking(root: &Option<Box<Node<i32>>>) -> Option<Vec<i32>> {
    let mut path = Vec::new();
    match &root {
        Some(node) => {
            if node.value == 0 {
                return None;
            }

            // This node is a leaf node
            if node.left.is_none() && node.right.is_none() {
                path.push(node.value);
                return Some(path);
            }

            match &mut backtracking(&node.right) {
                Some(sub_path) => {
                    path.push(node.value);
                    path.append(sub_path);
                    return Some(path);
                }
                None => {}
            };

            match &mut backtracking(&node.left) {
                Some(sub_path) => {
                    path.push(node.value);
                    path.append(sub_path);
                    return Some(path);
                }
                None => {}
            };

            None
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let result = backtracking(&Some(root));
        assert_eq!(result.unwrap(), [5, 7, 8]);
    }

    #[test]
    fn zero() {
        //             10
        //          9       11
        //      5               15
        //  0
        //      1
        let mut root = Box::new(Node::new(10));
        root.insert_left(9);
        root.insert_right(11);

        if let Some(node) = &mut root.left {
            // 9
            node.insert_left(5);

            if let Some(node) = &mut node.left {
                // 5
                node.insert_left(0);

                if let Some(node) = &mut node.left {
                    // 0
                    node.insert_right(1);
                }
            }
        }

        if let Some(node) = &mut root.right {
            node.insert_right(15);
        }

        let result = backtracking(&Some(root));
        assert_eq!(result.unwrap(), [10, 11, 15]);
    }
}
