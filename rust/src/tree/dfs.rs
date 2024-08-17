use super::node::Node;

fn dfs_inorder<T: Clone>(root: &Option<Box<Node<T>>>) -> Vec<T> {
    match root {
        Some(node) => {
            let mut result = dfs_inorder(&node.left);
            let mut right = dfs_inorder(&node.right);

            result.push(node.value.clone());
            result.append(&mut right);

            result
        }
        None => Vec::new(),
    }
}

fn dfs_preorder<T: Clone>(root: Option<Box<Node<T>>>) -> Vec<T> {
    match root {
        Some(node) => {
            let mut result = Vec::new();
            let mut left = dfs_inorder(&node.left);
            let mut right = dfs_inorder(&node.right);

            result.push(node.value);
            result.append(&mut left);
            result.append(&mut right);

            result
        }
        None => Vec::new(),
    }
}

fn dfs_postorder<T: Clone>(root: Option<Box<Node<T>>>) -> Vec<T> {
    match root {
        Some(node) => {
            let mut result = dfs_inorder(&node.left);
            let mut right = dfs_inorder(&node.right);

            result.append(&mut right);
            result.push(node.value);

            result
        }
        None => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::dfs_inorder;
    use super::dfs_preorder;
    use crate::tree::dfs::dfs_postorder;
    use crate::tree::node::Node;

    #[test]
    fn inorder() {
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

        let result = dfs_inorder(&Some(root));
        assert_eq!(result, [2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn preorder() {
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

        let result = dfs_preorder(Some(root));
        assert_eq!(result, [5, 2, 3, 4, 6, 7, 8]);
    }

    #[test]
    fn postorder() {
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

        let result = dfs_postorder(Some(root));
        assert_eq!(result, [2, 3, 4, 6, 7, 8, 5]);
    }
}
