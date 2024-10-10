/// Represents index of node in Graph.nodes vec.
pub type GraphNodeIndex = usize;

/// Graph representation.
/// - N is node type
/// - E is edge type
pub struct Graph<N, E> {
    // Represents nodes. Index of this vec is GraphNodeIndex.
    pub nodes: Vec<N>,
    /// Represents neighbours/edges of some node.
    /// Index of 'edges' corespond to index of 'nodes' and is GraphNodeIndex.
    /// An examples:
    /// - edges[2] has all neighbours/edges of node: nodes[2].
    /// - if node[k] is disconnected node then, edges[k] has empty Vec (i.e. no neighbours/edges)
    pub edges: Vec<Vec<(GraphNodeIndex, E)>>, // index of 'edges' is GraphNodeIndex, and represents
}

// Test graph:
//
//      g - b
//     /   / \
//    /   a - c
//   /   / \ /
//  f - e - d
//   \     /
//    \   /
//      h
//
#[cfg(test)]
pub(crate) fn make_test_graph() -> Graph<char, ()> {
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
