use std::{
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

use crate::data::graph::{Graph, GraphNode, GraphNodeIndex};

/// Search for shortest path in unweighted graph by using Breadth First Search.
pub fn bfs_shortest_path_in_indexed_graph<N, E>(
    graph: &Graph<N, E>,
    start: GraphNodeIndex,
    target: GraphNodeIndex,
) -> Vec<GraphNodeIndex> {
    let mut path = Vec::new();

    // bfs traverse until we reach target elemnet
    let mut fifo = VecDeque::new();
    let mut seen = vec![false; graph.nodes.len()];
    let mut prev = vec![None; graph.nodes.len()];

    fifo.push_back(start);
    seen[start] = true;

    'w: while let Some(current) = fifo.pop_front() {
        for &(neighbour, _) in &graph.edges[current] {
            if !seen[neighbour] {
                seen[neighbour] = true;
                fifo.push_back(neighbour);
                prev[neighbour] = Some(current);
                if neighbour == target {
                    break 'w;
                }
            }
        }
    }

    let mut current = target;
    path.push(current);
    while let Some(parent) = prev[current] {
        path.push(parent);
        current = parent;
    }
    path.reverse();

    path
}

pub fn bfs_shortest_path_in_linked_graph<V, E>(
    start: Rc<GraphNode<V, E>>,
    target: Rc<GraphNode<V, E>>,
) -> Vec<Rc<GraphNode<V, E>>> {
    let mut path = Vec::new();

    // bfs traverse until we reach target elemnet
    let mut previous = HashMap::new();
    let mut seen = HashSet::new();
    let mut fifo = VecDeque::new();

    fifo.push_back(start.clone());
    seen.insert(start.clone());
    previous.insert(start.clone(), None);

    'w: while let Some(current) = fifo.pop_front() {
        for (neighbour, _) in current.neighbours.borrow().iter() {
            if !seen.contains(neighbour) {
                seen.insert(neighbour.clone());
                fifo.push_back(neighbour.clone());
                previous.insert(neighbour.clone(), Some(current.clone()));
                if neighbour.clone() == target {
                    break 'w;
                }
            }
        }
    }

    path.push(target.clone());
    let mut current = target.clone();
    while let Some(parrent) = &previous[&current] {
        path.push(parrent.clone());
        current = parrent.clone();
    }
    path.reverse();

    path
}

#[cfg(test)]
mod tests {
    use crate::data::graph::{
        make_test_unweighted_indexed_graph, make_test_unweighted_linked_graph_nodes,
    };

    use super::*;

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
    fn bfs_shortest_path_works_for_indexed_graph() {
        let graph = make_test_unweighted_indexed_graph();

        // GraphNodeIndex:  0,   1,   2,   3,   4,   5,   6,   7
        //                 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'

        let shortest_path = bfs_shortest_path_in_indexed_graph(&graph, 0, 5);
        assert_eq!(shortest_path, [0, 4, 5]);
        let shortest_path = bfs_shortest_path_in_indexed_graph(&graph, 1, 5);
        assert_eq!(shortest_path, [1, 6, 5]);
        let shortest_path = bfs_shortest_path_in_indexed_graph(&graph, 0, 7);
        assert_eq!(shortest_path, [0, 3, 7]);
    }

    #[test] //bfs_shortest_path_in_linked_graph
    fn bfs_shortest_path_works_for_linked_graph() {
        let (start, end) = make_test_unweighted_linked_graph_nodes('a', Some('g'));
        let end = end.unwrap();
        let path = bfs_shortest_path_in_linked_graph(start.clone(), end.clone());

        let unique: HashSet<Rc<GraphNode<char, ()>>> = path.iter().cloned().collect();

        assert_eq!(unique.len(), path.len());
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], end);
        let expected = ['a', 'b', 'g'];
        for i in 0..(path.len() - 1) {
            let node = path[i].clone();
            let next = path[i + 1].clone();
            assert!(
                node.neighbours
                    .borrow()
                    .iter()
                    .any(|(neighbour, _)| *neighbour == next),
                "incorrect path"
            );
            assert!(node.value == expected[i]);
        }
    }
}
