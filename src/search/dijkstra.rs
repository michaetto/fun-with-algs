use crate::data::graph::{Graph, GraphNode, GraphNodeIndex};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

struct ShortestDistance {
    index: usize,
    value: u64,
}
impl PartialEq for ShortestDistance {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl PartialOrd for ShortestDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ShortestDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.partial_cmp(&self.value).unwrap()
    }
}
impl Eq for ShortestDistance {}

pub fn dijkstra_shortest_path_in_weighted_indexed_graph<V>(
    graph: &Graph<V, u64>,
    start: GraphNodeIndex,
    target: GraphNodeIndex,
) -> Vec<GraphNodeIndex> {
    let mut path = Vec::new();
    let mut distances = vec![u64::MAX; graph.nodes.len()];
    let mut heap = BinaryHeap::new(); // use min-heap to always follow shortest/cheapest choice
    let mut previous = vec![None; graph.nodes.len()];

    distances[start] = 0;
    heap.push(ShortestDistance {
        index: start,
        value: 0,
    });

    while let Some(ShortestDistance {
        index: current,
        value: current_distance,
    }) = heap.pop()
    {
        if current == target {
            break;
        }

        for &(neighbour, neighbour_edge) in &graph.edges[current] {
            let neighbour_distance = current_distance + neighbour_edge;
            if neighbour_distance < distances[neighbour] {
                let shortest_distance = ShortestDistance {
                    index: neighbour,
                    value: neighbour_distance,
                };
                heap.push(shortest_distance);
                distances[neighbour] = neighbour_distance;
                previous[neighbour] = Some(current);
            }
        }
    }

    path.push(target);
    let mut current = target;
    while let Some(parrent) = previous[current] {
        path.push(parrent);
        current = parrent;
    }
    path.reverse();

    let shortest_path_value = distances[target];
    println!("shortest_path_value: {shortest_path_value}");

    path
}

struct ShortestDistanceForGraphNode<V> {
    node: Rc<GraphNode<V, u64>>,
    value: u64,
}
impl<V> PartialEq for ShortestDistanceForGraphNode<V> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<V> PartialOrd for ShortestDistanceForGraphNode<V> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<V> Ord for ShortestDistanceForGraphNode<V> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.partial_cmp(&self.value).unwrap()
    }
}
impl<V> Eq for ShortestDistanceForGraphNode<V> {}

pub fn dijkstra_shortest_path_in_weighted_linked_graph<V>(
    start_node: Rc<GraphNode<V, u64>>,
    target_node: Rc<GraphNode<V, u64>>,
) -> Vec<Rc<GraphNode<V, u64>>> {
    let mut path = Vec::new();
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new(); // use min-heap to always follow shortest/cheapest choice
    let mut previous = HashMap::new();

    distances.insert(start_node.clone(), 0_u64);
    heap.push(ShortestDistanceForGraphNode {
        node: start_node.clone(),
        value: 0,
    });
    previous.insert(start_node.clone(), None);

    while let Some(ShortestDistanceForGraphNode {
        node: current_node,
        value: current_distance,
    }) = heap.pop()
    {
        if current_node == target_node {
            break;
        }

        for (neighbour_node, neighbour_edge) in current_node.neighbours.borrow().iter() {
            let neighbour_distance = current_distance + neighbour_edge;
            if neighbour_distance
                < distances
                    .get(&neighbour_node.clone())
                    .map_or(u64::MAX, |d| *d)
            {
                let shortest_distance = ShortestDistanceForGraphNode {
                    node: neighbour_node.clone(),
                    value: neighbour_distance,
                };
                heap.push(shortest_distance);
                distances.insert(neighbour_node.clone(), neighbour_distance);
                previous.insert(neighbour_node.clone(), Some(current_node.clone()));
            }
        }
    }

    path.push(target_node.clone());
    let mut current_node = target_node.clone();
    while let Some(parrent_node) = &previous[&current_node] {
        path.push(parrent_node.clone());
        current_node = parrent_node.clone();
    }

    path.reverse();

    let shortest_path_value = distances[&target_node];
    println!("shortest_path_value: {shortest_path_value}");
    path
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::data::graph::{
        make_test_weighted_indexed_graph, make_test_weighted_linked_graph_nodes,
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
    fn dijkstra_shortest_path_works() {
        let graph = make_test_weighted_indexed_graph();

        // GraphNodeIndex:  0,   1,   2,   3,   4,   5,   6,   7
        //                 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'

        let shortest_path = dijkstra_shortest_path_in_weighted_indexed_graph(&graph, 0, 5);
        assert_eq!(shortest_path, [0, 4, 5]);
        let shortest_path = dijkstra_shortest_path_in_weighted_indexed_graph(&graph, 1, 5);
        assert_eq!(shortest_path, [1, 0, 4, 5]);
        let shortest_path = dijkstra_shortest_path_in_weighted_indexed_graph(&graph, 0, 7);
        assert_eq!(shortest_path, [0, 3, 7]);
    }

    #[test]
    fn bfs_shortest_path_works_for_weighted_linked_graph() {
        // GraphNodeIndex:  0,   1,   2,   3,   4,   5,   6,   7
        //                 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'

        // 'a' -> 'f', i.e. 0->5
        let (start, end) = make_test_weighted_linked_graph_nodes('a', Some('f'));
        let end = end.unwrap();
        let path = dijkstra_shortest_path_in_weighted_linked_graph(start.clone(), end.clone());
        let unique: HashSet<Rc<GraphNode<char, u64>>> = path.iter().cloned().collect();
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
        assert_eq!(
            path.iter().map(|n| n.value).collect::<Vec<_>>(),
            ['a', 'e', 'f']
        );

        // 'b' -> 'f', i.e. 1->5
        let (start, end) = make_test_weighted_linked_graph_nodes('b', Some('f'));
        let end = end.unwrap();
        let path = dijkstra_shortest_path_in_weighted_linked_graph(start.clone(), end.clone());
        let unique: HashSet<Rc<GraphNode<char, u64>>> = path.iter().cloned().collect();
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
        assert_eq!(
            path.iter().map(|n| n.value).collect::<Vec<_>>(),
            ['b', 'a', 'e', 'f']
        );

        // 'a' -> 'h', i.e. 0->7
        let (start, end) = make_test_weighted_linked_graph_nodes('a', Some('h'));
        let end = end.unwrap();
        let path = dijkstra_shortest_path_in_weighted_linked_graph(start.clone(), end.clone());
        let unique: HashSet<Rc<GraphNode<char, u64>>> = path.iter().cloned().collect();
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
        assert_eq!(
            path.iter().map(|n| n.value).collect::<Vec<_>>(),
            ['a', 'd', 'h']
        );
    }
}
