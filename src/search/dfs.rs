use crate::data::graph::{Graph, GraphNodeIndex};

/// Search for some path in graph by using Depth First Search.
pub fn dfs_path<N, E>(
    graph: &Graph<N, E>,
    start: GraphNodeIndex,
    end: GraphNodeIndex,
) -> Vec<GraphNodeIndex> {
    let mut lifo: Vec<GraphNodeIndex> = Vec::new();
    let mut seen: Vec<bool> = vec![false; graph.nodes.len()];
    let mut prev: Vec<Option<GraphNodeIndex>> = vec![None; graph.nodes.len()];

    lifo.push(start);

    let mut previous = None;

    'w: while let Some(current) = lifo.pop() {
        if !seen[current] {
            prev[current] = previous;
            seen[current] = true;
            previous = Some(current);
            if current == end {
                break 'w;
            }
        }
        for &(neighbour, _) in &graph.edges[current] {
            if !seen[neighbour] {
                lifo.push(neighbour);
            }
        }
    }
    let mut path = Vec::new();
    path.push(end);
    let mut current = end;
    while let Some(parent) = prev[current] {
        path.push(parent);
        current = parent;
    }
    path.reverse();
    path
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
    fn dfs_path_works() {
        let graph = make_test_graph();
        let start = 0;
        let end = 6;
        let path = dfs_path(&graph, start, end);

        let unique: HashSet<GraphNodeIndex> = path.iter().copied().collect();
        assert_eq!(unique.len(), path.len());
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], end);
        for i in 0..(path.len() - 1) {
            let node_index = path[i];
            let next_index = path[i + 1];
            assert!(
                graph.edges[node_index]
                    .iter()
                    .any(|&(neighbour, _)| neighbour == next_index),
                "incorrect path"
            );
        }
    }
}
