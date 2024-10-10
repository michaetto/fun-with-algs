use crate::data::graph::{Graph, GraphNodeIndex};

/// Traverse whole graph (by visiting all nodes) by using Depth First Search.
pub fn dfs_traversal<N, E>(graph: &Graph<N, E>, start: GraphNodeIndex) -> Vec<GraphNodeIndex> {
    let mut output: Vec<GraphNodeIndex> = Vec::new();
    let mut lifo: Vec<GraphNodeIndex> = Vec::new();
    let mut seen: Vec<bool> = vec![false; graph.nodes.len()];

    lifo.push(start);

    while let Some(current) = lifo.pop() {
        if !seen[current] {
            output.push(current);
            seen[current] = true;
        }
        for &(neighbour, _) in &graph.edges[current] {
            if !seen[neighbour] {
                lifo.push(neighbour);
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
    fn dfs_traversal_works() {
        let graph = make_test_graph();
        let dfs_from_a_indexes = dfs_traversal(&graph, 0);

        assert_eq!(dfs_from_a_indexes.len(), graph.nodes.len());
        let unique: HashSet<GraphNodeIndex> = dfs_from_a_indexes.into_iter().collect();
        assert_eq!(unique.len(), graph.nodes.len());
    }
}
