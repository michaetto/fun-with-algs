/// Pre/In/Post order binary tree traversal.
use crate::data::binary_tree::Node;

/// Pre Order Traversal.
pub fn pre_order_traversal_values<T: Clone>(tree: impl Node<T>) -> Vec<T> {
    let mut output = Vec::new();
    n_l_r(&mut output, &tree);
    output
}
fn n_l_r<T: Clone>(output: &mut Vec<T>, node: &impl Node<T>) {
    output.push(node.value().clone());
    if let Some(left) = node.left() {
        n_l_r(output, left)
    }
    if let Some(right) = node.right() {
        n_l_r(output, right)
    }
}

/// In Order Traversal.
pub fn in_order_traversal_values<T: Clone>(tree: impl Node<T>) -> Vec<T> {
    let mut output = Vec::new();
    l_n_r(&mut output, &tree);
    output
}
fn l_n_r<T: Clone>(output: &mut Vec<T>, node: &impl Node<T>) {
    if let Some(left) = node.left() {
        l_n_r(output, left)
    }

    output.push(node.value().clone());

    if let Some(right) = node.right() {
        l_n_r(output, right)
    }
}

/// Post Order Traversal.
pub fn post_order_traversal_values<T: Clone>(tree: impl Node<T>) -> Vec<T> {
    let mut output = Vec::new();
    l_r_n(&mut output, &tree);
    output
}
fn l_r_n<T: Clone>(output: &mut Vec<T>, node: &impl Node<T>) {
    if let Some(left) = node.left() {
        l_r_n(output, left)
    }

    if let Some(right) = node.right() {
        l_r_n(output, right)
    }

    output.push(node.value().clone());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::binary_tree::make_test_nodetree;

    #[test]
    fn pre_order_traversal_works() {
        let root = make_test_nodetree();
        let output = pre_order_traversal_values(root);
        let expected = vec![10, 7, 6, 1, 8, 9, 11, 20, 14, 22];
        assert_eq!(expected, output);
    }

    #[test]
    fn in_order_traversal_works() {
        let root = make_test_nodetree();
        let output = in_order_traversal_values(root);
        let expected = vec![1, 6, 7, 8, 9, 10, 11, 14, 20, 22];
        assert_eq!(expected, output);
    }

    #[test]
    fn post_order_traversal_works() {
        let root = make_test_nodetree();
        let output = post_order_traversal_values(root);
        let expected = vec![1, 6, 9, 8, 7, 14, 22, 20, 11, 10];
        assert_eq!(expected, output);
    }
}
