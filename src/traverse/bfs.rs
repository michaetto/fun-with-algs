use crate::data::graph::{Graph, GraphNodeIndex};
use std::collections::VecDeque;

/// Traverse whole graph (by visiting all nodes) by using Breadth First Search.
pub fn bfs_traversal<N, E>(graph: &Graph<N, E>, start: GraphNodeIndex) -> Vec<GraphNodeIndex> {
    let mut fifo: VecDeque<GraphNodeIndex> = VecDeque::new();
    let mut seen: Vec<bool> = vec![false; graph.nodes.len()];
    let mut output: Vec<GraphNodeIndex> = Vec::new();

    fifo.push_back(start);
    seen[start] = true;
    output.push(start);

    while let Some(current) = fifo.pop_front() {
        for &(neighbour, _) in &graph.edges[current] {
            if !seen[neighbour] {
                fifo.push_back(neighbour);
                seen[neighbour] = true;
                output.push(neighbour);
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::graph::make_test_graph;
    use std::collections::HashSet;

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
    #[test]
    fn bfs_traversal_works() {
        let graph = make_test_graph();
        let bfs_from_a_indexes = bfs_traversal(&graph, 0);

        assert_eq!(bfs_from_a_indexes.len(), graph.nodes.len());
        let unique: HashSet<GraphNodeIndex> = bfs_from_a_indexes.into_iter().collect();
        assert_eq!(unique.len(), graph.nodes.len());
    }
}
