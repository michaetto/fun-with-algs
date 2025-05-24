use crate::data::graph::{Graph, GraphNodeIndex};

/// Traverse whole graph (by visiting all nodes) by using Depth First Search.
pub fn dfs_graph_traversal<V, E>(
    graph: &Graph<V, E>,
    start: GraphNodeIndex,
) -> Vec<GraphNodeIndex> {
    let mut output = Vec::new();
    let mut lifo = Vec::new();
    let mut seen = vec![false; graph.nodes.len()];

    lifo.push(start);

    while let Some(current) = lifo.pop() {
        if !seen[current] {
            seen[current] = true;
            output.push(current);
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
    use crate::data::graph::make_test_unweighted_indexed_graph;

    use super::*;
    use std::collections::HashSet;

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
    #[test]
    fn dfs_traversal_works() {
        let graph = make_test_unweighted_indexed_graph();
        let dfs_from_a_indexes = dfs_graph_traversal(&graph, 0);

        assert_eq!(dfs_from_a_indexes.len(), graph.nodes.len());
        let unique: HashSet<GraphNodeIndex> = dfs_from_a_indexes.into_iter().collect();
        assert_eq!(unique.len(), graph.nodes.len());
    }
}
