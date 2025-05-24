/// Binary Tree abstract interface.
pub trait Node<T: Clone> {
    fn left(&self) -> Option<&impl Node<T>>;
    fn right(&self) -> Option<&impl Node<T>>;
    fn value(&self) -> &T;
}

/// Binary Tree representation as nodes with children.
pub struct NodeTree<T: Clone> {
    left: Option<Box<NodeTree<T>>>,
    right: Option<Box<NodeTree<T>>>,
    value: T,
}
impl<T: Clone> Node<T> for NodeTree<T> {
    fn left(&self) -> Option<&impl Node<T>> {
        self.left.as_ref().map(|left| left.as_ref())
    }

    fn right(&self) -> Option<&impl Node<T>> {
        self.right.as_ref().map(|right| right.as_ref())
    }

    fn value(&self) -> &T {
        &self.value
    }
}

// Test tree:
//          10
//          / \
//         7   11
//        / \    \
//       6   8    20
//      /     \   / \
//     1       9 14 22
#[cfg(test)]
pub(crate) fn make_test_nodetree() -> NodeTree<usize> {
    NodeTree {
        value: 10,
        left: Some(Box::new(NodeTree {
            value: 7,
            left: Some(Box::new(NodeTree {
                value: 6,
                left: Some(Box::new(NodeTree {
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            right: Some(Box::new(NodeTree {
                value: 8,
                left: None,
                right: Some(Box::new(NodeTree {
                    value: 9,
                    left: None,
                    right: None,
                })),
            })),
        })),
        right: Some(Box::new(NodeTree {
            value: 11,
            left: None,
            right: Some(Box::new(NodeTree {
                value: 20,
                left: Some(Box::new(NodeTree {
                    value: 14,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(NodeTree {
                    value: 22,
                    left: None,
                    right: None,
                })),
            })),
        })),
    }
}

// TODO: another (e.g. flat array) representation of tree by implementing Node trait ?
// flat array as:
// p - parent index
// 2*p + 1 - left_child index
// 2*p + 2 - right child index
