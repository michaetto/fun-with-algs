use crate::data::graph::{Graph, GraphNodeIndex};
use std::collections::VecDeque;

/// Search for shortest path in unweighted graph by using Breadth First Search.
pub fn bfs_shortest_path_for_unweighted_graph<N, E>(
    graph: &Graph<N, E>,
    start: GraphNodeIndex,
    end: GraphNodeIndex,
) -> Vec<GraphNodeIndex> {
    let mut fifo: VecDeque<GraphNodeIndex> = VecDeque::new();
    let mut seen: Vec<bool> = vec![false; graph.nodes.len()];
    let mut prev = vec![None; graph.nodes.len()];

    fifo.push_back(start);
    seen[start] = true;

    'w: while let Some(current) = fifo.pop_front() {
        for &(neighbour, _) in &graph.edges[current] {
            if !seen[neighbour] {
                fifo.push_back(neighbour);
                seen[neighbour] = true;
                prev[neighbour] = Some(current);
                if neighbour == end {
                    break 'w;
                }
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
    fn bfs_shortest_path_works() {
        let graph = make_test_graph();

        // GraphNodeIndex:  0,   1,   2,   3,   4,   5,   6,   7
        //                 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'

        let shortest_path = bfs_shortest_path_for_unweighted_graph(&graph, 0, 5);
        assert_eq!(shortest_path, [0, 4, 5]);
        let shortest_path = bfs_shortest_path_for_unweighted_graph(&graph, 1, 5);
        assert_eq!(shortest_path, [1, 6, 5]);
        let shortest_path = bfs_shortest_path_for_unweighted_graph(&graph, 0, 7);
        assert_eq!(shortest_path, [0, 3, 7]);
    }
}
