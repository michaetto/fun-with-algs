use crate::data::graph::GraphNodeIndex;

/// Maze positions:
/// - i increase from top to bottom
/// - j increses from left to right
/// - An example:
///   {i:0,j:0}, {i:0,j:1}, {i:0,j:2}
///   {i:1,j:0}, {i:1,j:1}, {i:1,j:2}
///   {i:2,j:0}, {i:2,j:1}, {i:2,j:2}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Position {
    pub i: usize,
    pub j: usize,
}

/// Movements (by vector).
///        (i , j)
/// up:    (-1, 0)
/// right: (0 , 1)
/// down:  (1 , 0)
/// left:  (0 ,-1)
impl Position {
    pub fn new(i: usize, j: usize) -> Self {
        Position { i, j }
    }
    pub fn up(self) -> Option<Position> {
        self.i.checked_sub(1).map(|i| Position { i, j: self.j })
    }
    pub fn right(self, maze_width: usize) -> Option<Position> {
        self.j
            .checked_add(1)
            .filter(|&j| j < maze_width)
            .map(|j| Position { i: self.i, j })
    }
    pub fn down(self, maze_height: usize) -> Option<Position> {
        self.i
            .checked_add(1)
            .filter(|&i| i < maze_height)
            .map(|i| Position { i, j: self.j })
    }
    pub fn left(self) -> Option<Position> {
        self.j.checked_sub(1).map(|j| Position { i: self.i, j })
    }

    // index: {position}
    //     0: {i:0,j:0}, 1: {i:0,j:1}, 2: {i:0,j:2}
    //     3: {i:1,j:0}, 4: {i:1,j:1}, 5: {i:1,j:2}
    //     6: {i:2,j:0}, 7: {i:2,j:1}, 8: {i:2,j:2}

    /// Convert position to index (having maze_width).
    pub fn index(self, maze_width: usize) -> usize {
        let index = self.i * maze_width + self.j;
        debug_assert!(index < 64, "node_position: {:?}", self);
        index
    }
    /// Convert index to position (having maze_width).
    pub fn from_index(index: GraphNodeIndex, maze_width: usize) -> Self {
        let i = index / maze_width;
        let j = index % maze_width;
        debug_assert!(i < maze_width && j < maze_width, "i: {i}, j: {j}");
        Position { i, j }
    }
}

#[cfg(test)]
use crate::data::graph::Graph;

// Test maze to test graph:
// '1' means wall
//
//        10100010->end
//        00100100
//        01101101         convert
//        01100101         ------>  unweighted graph where:
//        01110101                  - each '1' (wall) is just disconnected node (with Position)
//        00110100                  - each '0' is connected to all surrounding '0' (surrouding means only: up/down/right/left)
//        10000010
// start->00111000
#[cfg(test)]
fn construct_test_maze_as_graph() -> Graph<Position, ()> {
    let mut graph = Graph {
        nodes: Vec::new(),
        edges: Vec::new(),
    };
    // '1' means wall
    let maze = vec![
        vec![1, 0, 1, 0, 0, 0, 1, 0],
        vec![0, 0, 1, 0, 0, 1, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 1],
        vec![0, 1, 1, 0, 0, 1, 0, 1],
        vec![0, 1, 1, 1, 0, 1, 0, 1],
        vec![0, 0, 1, 1, 0, 1, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 1, 1, 1, 0, 0, 0],
    ];

    let height = maze.len();
    let width = maze[0].len();
    for i in 0..height {
        for j in 0..width {
            // node is Some only if it is '0', i.e. part of path
            // node is None if it is a wall
            let node_position = Position::new(i, j);
            let node_index: GraphNodeIndex = graph.nodes.len(); // This is equivalent to node_position.index(width), due to the order of looping over indexes 'i' and 'j' and the fact that we also add disconnected nodes
            debug_assert!(node_index < 64, "node_position: {node_position:?}");
            graph.nodes.push(node_position);
            graph.edges.push(Vec::new());

            // walls are just disconnected nodes in graph
            if maze[node_position.i][node_position.j] == 1 {
                continue;
            }

            // Construct edges/neighbours only when there is actual path through maze
            // i.e. corresponding neighour is '0'
            if let Some(neighbour_position) = node_position.up() {
                if maze[neighbour_position.i][neighbour_position.j] == 0 {
                    let neighbour_index = neighbour_position.index(width);
                    graph.edges[node_index].push((neighbour_index, ()));
                }
            }
            if let Some(neighbour_position) = node_position.right(width) {
                if maze[neighbour_position.i][neighbour_position.j] == 0 {
                    let neighbour_index = neighbour_position.index(width);
                    graph.edges[node_index].push((neighbour_index, ()));
                }
            }
            if let Some(neighbour_position) = node_position.down(height) {
                if maze[neighbour_position.i][neighbour_position.j] == 0 {
                    let neighbour_index = neighbour_position.index(width);
                    graph.edges[node_index].push((neighbour_index, ()));
                }
            }
            if let Some(neighbour_position) = node_position.left() {
                if maze[neighbour_position.i][neighbour_position.j] == 0 {
                    let neighbour_index = neighbour_position.index(width);
                    graph.edges[node_index].push((neighbour_index, ()));
                }
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use crate::search::bfs::bfs_shortest_path_in_indexed_graph;

    use super::*;

    // Maze:
    //        10100010<-end
    //        00100100
    //        01101101
    //        01100101
    //        01110101
    //        00110100
    //        10000010
    // start->00111000
    #[test]
    fn solve_maze_using_bfs() {
        let maze = construct_test_maze_as_graph();
        let maze_width = 8;
        let start_position = Position { i: 7, j: 0 };
        assert_eq!(
            start_position,
            Position::from_index(start_position.index(maze_width), maze_width)
        );
        let end_position = Position { i: 0, j: 7 };
        assert_eq!(
            end_position,
            Position::from_index(end_position.index(maze_width), maze_width)
        );
        let found_path = bfs_shortest_path_in_indexed_graph(
            &maze,
            start_position.index(maze_width),
            end_position.index(maze_width),
        );
        assert!(found_path.len() == 19);

        let expected_path = vec![
            Position { i: 7, j: 0 },
            Position { i: 7, j: 1 },
            Position { i: 6, j: 1 },
            Position { i: 6, j: 2 },
            Position { i: 6, j: 3 },
            Position { i: 6, j: 4 },
            Position { i: 6, j: 5 },
            Position { i: 7, j: 5 },
            Position { i: 7, j: 6 },
            Position { i: 7, j: 7 },
            Position { i: 6, j: 7 },
            Position { i: 5, j: 7 },
            Position { i: 5, j: 6 },
            Position { i: 4, j: 6 },
            Position { i: 3, j: 6 },
            Position { i: 2, j: 6 },
            Position { i: 1, j: 6 },
            Position { i: 1, j: 7 },
            Position { i: 0, j: 7 },
        ];
        assert_eq!(
            expected_path,
            found_path
                .into_iter()
                .map(|index| Position::from_index(index, maze_width))
                .collect::<Vec<Position>>()
        );
    }
}
