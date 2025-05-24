use std::{cell::RefCell, rc::Rc};

/// Represents index of node in Graph.nodes vec.
pub type GraphNodeIndex = usize;

/// Indexed graph representation.
/// - V is node value type
/// - E is edge type
pub struct Graph<V, E> {
    // Represents nodes. Index of this vec is GraphNodeIndex.
    pub nodes: Vec<V>,
    /// Represents neighbours/edges of some node.
    /// Index of 'edges' corespond to index of 'nodes' and is GraphNodeIndex.
    /// An examples:
    /// - edges[2] has all neighbours/edges of node: nodes[2].
    /// - if node[k] is disconnected node then, edges[k] has empty Vec (i.e. no neighbours/edges)
    pub edges: Vec<Vec<(GraphNodeIndex, E)>>, // index of 'edges' is GraphNodeIndex, and represents
}

use std::hash::Hash;

/// Linked nodes graph representation.
#[derive(Debug)]
pub struct GraphNode<V, E> {
    pub value: V,
    pub neighbours: RefCell<Vec<(Rc<GraphNode<V, E>>, E)>>,
}

impl<V, E> PartialEq for GraphNode<V, E> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(&self.value, &other.value)
    }
}
impl<V, E> Eq for GraphNode<V, E> {}
impl<V, E> Hash for GraphNode<V, E> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let ptr = &self.value as *const V;
        ptr.hash(state);
    }
}

///////////--- Test graph --- /////////
//                 //                //
//      g - b      //      6 - 1     //
//     /   / \     //     /   / \    //
//    /   a - c    //    /   0 - 2   //
//   /   / \ /     //   /   / \ /    //
//  f - e - d      //  5 - 4 - 3     //
//   \     /       //   \     /      //
//    \   /        //    \   /       //
//      h          //      7         //
//                 //                //
///////////////////////////////////////
#[cfg(test)]
pub fn make_test_unweighted_indexed_graph() -> Graph<char, ()> {
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };
    // GraphNodeIndex:  0,   1,   2,   3,   4,   5,   6,   7
    graph.nodes = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    // 'a' neigbours
    graph
        .edges
        .insert(0, vec![(1, ()), (2, ()), (3, ()), (4, ())]);

    // 'b' neigbours
    graph.edges.insert(1, vec![(6, ()), (0, ()), (2, ())]);

    // 'c' neigbours
    graph.edges.insert(2, vec![(3, ()), (0, ()), (1, ())]);

    // 'd' neigbours
    graph
        .edges
        .insert(3, vec![(2, ()), (0, ()), (4, ()), (7, ())]);

    // 'e' neigbours
    graph.edges.insert(4, vec![(0, ()), (3, ()), (5, ())]);

    // 'f' neigbours
    graph.edges.insert(5, vec![(6, ()), (4, ()), (7, ())]);

    // 'g' neigbours
    graph.edges.insert(6, vec![(5, ()), (1, ())]);

    // 'h' neigbours
    graph.edges.insert(7, vec![(5, ()), (3, ())]);

    graph
}

#[cfg(test)]
pub fn make_test_weighted_indexed_graph() -> Graph<char, u64> {
    let mut graph: Graph<char, u64> = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };
    // GraphNodeIndex:  0,   1,   2,   3,   4,   5,   6,   7
    graph.nodes = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

    // 'a' neigbours
    graph.edges.insert(0, vec![(1, 1), (2, 1), (3, 1), (4, 1)]);

    // 'b' neigbours
    graph.edges.insert(1, vec![(6, 1), (0, 1), (2, 1)]);

    // 'c' neigbours
    graph.edges.insert(2, vec![(3, 1), (0, 1), (1, 1)]);

    // 'd' neigbours
    graph.edges.insert(3, vec![(2, 1), (0, 1), (4, 1), (7, 2)]);

    // 'e' neigbours
    graph.edges.insert(4, vec![(0, 1), (3, 1), (5, 1)]);

    // 'f' neigbours
    graph.edges.insert(5, vec![(6, 3), (4, 1), (7, 2)]);

    // 'g' neigbours
    graph.edges.insert(6, vec![(5, 3), (1, 1)]);

    // 'h' neigbours
    graph.edges.insert(7, vec![(5, 2), (3, 2)]);

    graph
}

#[cfg(test)]
pub fn make_test_unweighted_linked_graph_nodes(
    start: char,
    end: Option<char>,
) -> (Rc<GraphNode<char, ()>>, Option<Rc<GraphNode<char, ()>>>) {
    let a = Rc::new(GraphNode {
        value: 'a',
        neighbours: RefCell::new(vec![]),
    });
    let b = Rc::new(GraphNode {
        value: 'b',
        neighbours: RefCell::new(vec![]),
    });
    let c = Rc::new(GraphNode {
        value: 'c',
        neighbours: RefCell::new(vec![]),
    });
    let d = Rc::new(GraphNode {
        value: 'd',
        neighbours: RefCell::new(vec![]),
    });
    let e = Rc::new(GraphNode {
        value: 'e',
        neighbours: RefCell::new(vec![]),
    });
    let f = Rc::new(GraphNode {
        value: 'f',
        neighbours: RefCell::new(vec![]),
    });
    let g = Rc::new(GraphNode {
        value: 'g',
        neighbours: RefCell::new(vec![]),
    });
    let h = Rc::new(GraphNode {
        value: 'h',
        neighbours: RefCell::new(vec![]),
    });

    // 'a' neigbours
    (*a.neighbours.borrow_mut()).append(&mut vec![
        (b.clone(), ()),
        (c.clone(), ()),
        (d.clone(), ()),
        (e.clone(), ()),
    ]);

    // 'b' neigbours
    (*b.neighbours.borrow_mut()).append(&mut vec![
        (a.clone(), ()),
        (g.clone(), ()),
        (c.clone(), ()),
    ]);

    // 'c' neigbours
    (*c.neighbours.borrow_mut()).append(&mut vec![
        (a.clone(), ()),
        (b.clone(), ()),
        (d.clone(), ()),
    ]);

    // 'd' neigbours
    (*d.neighbours.borrow_mut()).append(&mut vec![
        (c.clone(), ()),
        (a.clone(), ()),
        (e.clone(), ()),
        (h.clone(), ()),
    ]);

    // 'e' neigbours
    (*e.neighbours.borrow_mut()).append(&mut vec![
        (a.clone(), ()),
        (d.clone(), ()),
        (f.clone(), ()),
    ]);

    // 'f' neigbours
    (*f.neighbours.borrow_mut()).append(&mut vec![
        (g.clone(), ()),
        (e.clone(), ()),
        (h.clone(), ()),
    ]);

    // 'g' neigbours
    (*g.neighbours.borrow_mut()).append(&mut vec![(f.clone(), ()), (b.clone(), ())]);

    // 'h' neigbours
    (*h.neighbours.borrow_mut()).append(&mut vec![(f.clone(), ()), (d.clone(), ())]);

    let start = match start {
        'a' => a.clone(),
        'b' => b.clone(),
        'c' => c.clone(),
        'd' => d.clone(),
        'e' => e.clone(),
        'f' => f.clone(),
        'g' => g.clone(),
        'h' => h.clone(),
        _ => panic!("invlid start"),
    };

    let end = match end {
        Some('a') => Some(a.clone()),
        Some('b') => Some(b.clone()),
        Some('c') => Some(c.clone()),
        Some('d') => Some(d.clone()),
        Some('e') => Some(e.clone()),
        Some('f') => Some(f.clone()),
        Some('g') => Some(g.clone()),
        Some('h') => Some(h.clone()),
        _ => None,
    };

    (start, end)
}

#[cfg(test)]
pub fn make_test_weighted_linked_graph_nodes(
    start: char,
    end: Option<char>,
) -> (Rc<GraphNode<char, u64>>, Option<Rc<GraphNode<char, u64>>>) {
    let a = Rc::new(GraphNode {
        value: 'a',
        neighbours: RefCell::new(vec![]),
    });
    let b = Rc::new(GraphNode {
        value: 'b',
        neighbours: RefCell::new(vec![]),
    });
    let c = Rc::new(GraphNode {
        value: 'c',
        neighbours: RefCell::new(vec![]),
    });
    let d = Rc::new(GraphNode {
        value: 'd',
        neighbours: RefCell::new(vec![]),
    });
    let e = Rc::new(GraphNode {
        value: 'e',
        neighbours: RefCell::new(vec![]),
    });
    let f = Rc::new(GraphNode {
        value: 'f',
        neighbours: RefCell::new(vec![]),
    });
    let g = Rc::new(GraphNode {
        value: 'g',
        neighbours: RefCell::new(vec![]),
    });
    let h = Rc::new(GraphNode {
        value: 'h',
        neighbours: RefCell::new(vec![]),
    });

    // 'a' neigbours
    (*a.neighbours.borrow_mut()).append(&mut vec![
        (b.clone(), 1),
        (c.clone(), 1),
        (d.clone(), 1),
        (e.clone(), 1),
    ]);

    // 'b' neigbours
    (*b.neighbours.borrow_mut()).append(&mut vec![(a.clone(), 1), (g.clone(), 1), (c.clone(), 1)]);

    // 'c' neigbours
    (*c.neighbours.borrow_mut()).append(&mut vec![(a.clone(), 1), (b.clone(), 1), (d.clone(), 1)]);

    // 'd' neigbours
    (*d.neighbours.borrow_mut()).append(&mut vec![
        (c.clone(), 1),
        (a.clone(), 1),
        (e.clone(), 1),
        (h.clone(), 2),
    ]);

    // 'e' neigbours
    (*e.neighbours.borrow_mut()).append(&mut vec![(a.clone(), 1), (d.clone(), 1), (f.clone(), 1)]);

    // 'f' neigbours
    (*f.neighbours.borrow_mut()).append(&mut vec![(g.clone(), 3), (e.clone(), 1), (h.clone(), 2)]);

    // 'g' neigbours
    (*g.neighbours.borrow_mut()).append(&mut vec![(f.clone(), 3), (b.clone(), 1)]);

    // 'h' neigbours
    (*h.neighbours.borrow_mut()).append(&mut vec![(f.clone(), 2), (d.clone(), 2)]);

    let start = match start {
        'a' => a.clone(),
        'b' => b.clone(),
        'c' => c.clone(),
        'd' => d.clone(),
        'e' => e.clone(),
        'f' => f.clone(),
        'g' => g.clone(),
        'h' => h.clone(),
        _ => panic!("invlid start"),
    };

    let end = match end {
        Some('a') => Some(a.clone()),
        Some('b') => Some(b.clone()),
        Some('c') => Some(c.clone()),
        Some('d') => Some(d.clone()),
        Some('e') => Some(e.clone()),
        Some('f') => Some(f.clone()),
        Some('g') => Some(g.clone()),
        Some('h') => Some(h.clone()),
        _ => None,
    };

    (start, end)
}
