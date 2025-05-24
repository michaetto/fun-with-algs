use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::data::graph::{Graph, GraphNode, GraphNodeIndex};

pub fn dfs_path_in_unweighted_indexed_graph<V, E>(
    graph: &Graph<V, E>,
    start: GraphNodeIndex,
    target: GraphNodeIndex,
) -> Vec<GraphNodeIndex> {
    let mut path = Vec::new();
    let mut lifo = Vec::new();
    let mut seen = vec![false; graph.nodes.len()];
    let mut previous: Vec<Option<GraphNodeIndex>> = vec![None; graph.nodes.len()];
    let mut prev = None;

    lifo.push(start);

    while let Some(current) = lifo.pop() {
        if !seen[current] {
            previous[current] = prev;
            prev = Some(current); // for next iteration of th loop

            seen[current] = true;
            if current == target {
                break;
            }
        }

        for &(neighbour, _) in &graph.edges[current] {
            if !seen[neighbour] {
                lifo.push(neighbour);
            }
        }
    }

    path.push(target);
    let mut current = target;
    while let Some(parent) = previous[current] {
        path.push(parent);
        current = parent
    }
    path.reverse();

    path
}

fn dfs_path_in_unweighted_linked_graph<V, E>(
    start: Rc<GraphNode<V, E>>,
    target: Rc<GraphNode<V, E>>,
) -> Vec<Rc<GraphNode<V, E>>> {
    let mut path = Vec::new();
    let mut previous = HashMap::new();
    let mut lifo = Vec::new();
    let mut seen = HashSet::new();

    lifo.push(start.clone());

    let mut prev = None;
    while let Some(current) = lifo.pop() {
        if !seen.contains(&current) {
            previous.insert(current.clone(), prev);
            seen.insert(current.clone());
            prev = Some(current.clone());
            if current == target {
                break;
            }
        }

        for (neighbour, _) in current.neighbours.borrow().iter() {
            if !seen.contains(neighbour) {
                lifo.push(neighbour.clone())
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
    fn dfs_path_works_for_unweighted_indexed_graph() {
        let graph = make_test_unweighted_indexed_graph();
        let start = 0;
        let end = 6;
        let path = dfs_path_in_unweighted_indexed_graph(&graph, start, end);

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

    #[test]
    fn dfs_path_works_for_unweighted_linked_graph() {
        let (start, end) = make_test_unweighted_linked_graph_nodes('a', Some('g'));
        let end = end.unwrap();
        let path = dfs_path_in_unweighted_linked_graph(start.clone(), end.clone());

        let unique: HashSet<Rc<GraphNode<char, ()>>> = path.iter().cloned().collect();

        assert_eq!(unique.len(), path.len());
        assert_eq!(path[0], start);
        assert_eq!(path[path.len() - 1], end);
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
        }
    }
}
