use crate::data::graph::{Graph, GraphNode, GraphNodeIndex};
use std::{
    collections::{HashSet, VecDeque},
    rc::Rc,
};

/// Traverse whole graph (by visiting all nodes) by using Breadth First Search.
fn bfs_graph_traversal_in_indexed_graph<N, E>(
    graph: &Graph<N, E>,
    start: GraphNodeIndex,
) -> Vec<GraphNodeIndex> {
    let mut fifo = VecDeque::new();
    let mut output = Vec::new();
    let mut seen = vec![false; graph.nodes.len()];

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

fn bfs_graph_traversal_in_linked_graph(
    start_node: Rc<GraphNode<char, ()>>,
) -> Vec<Rc<GraphNode<char, ()>>> {
    let mut output = Vec::new();
    let mut fifo = VecDeque::new();
    let mut seen = HashSet::new();

    fifo.push_back(start_node.clone());
    seen.insert(start_node.clone());
    output.push(start_node.clone());

    while let Some(node) = fifo.pop_front() {
        for (neighbour, _edge) in node.neighbours.borrow().iter() {
            if !seen.contains(neighbour) {
                seen.insert(neighbour.clone());
                fifo.push_back(neighbour.clone());
                output.push(neighbour.clone());
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::data::graph::{
        make_test_unweighted_indexed_graph, make_test_unweighted_linked_graph_nodes,
    };

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
    fn bfs_graph_traversal_works() {
        let graph = make_test_unweighted_indexed_graph();
        let bfs_from_a_indexes = bfs_graph_traversal_in_indexed_graph(&graph, 0);

        assert_eq!(bfs_from_a_indexes.len(), graph.nodes.len());
        let unique: HashSet<GraphNodeIndex> = bfs_from_a_indexes.into_iter().collect();
        assert_eq!(unique.len(), graph.nodes.len());
    }

    #[test]
    fn bfs_graph_node_traversal_works() {
        let (graph_node, _) = make_test_unweighted_linked_graph_nodes('a', None);
        let bfs_from_a_indexes = bfs_graph_traversal_in_linked_graph(graph_node);

        assert_eq!(bfs_from_a_indexes.len(), 8);
        let unique: HashSet<Rc<GraphNode<char, ()>>> = bfs_from_a_indexes.into_iter().collect();
        assert_eq!(unique.len(), 8);
    }
}
