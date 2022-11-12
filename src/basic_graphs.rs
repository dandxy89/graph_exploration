use std::{
    collections::{HashSet, VecDeque},
    fmt::{Display, Formatter},
};

#[derive(PartialEq, Eq, Hash)]
pub struct Edge(usize, usize);

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(From={}, To={})", self.0, self.1)
    }
}

impl From<(usize, usize)> for Edge {
    fn from(pair: (usize, usize)) -> Self {
        Self(pair.0, pair.1)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Node(usize);

impl From<usize> for Node {
    fn from(v: usize) -> Self {
        Self(v)
    }
}

impl Node {
    pub fn neighbours(&self, g: &Graph) -> VecDeque<Node> {
        g.edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

#[allow(dead_code)]
pub struct Graph<'a> {
    edges: &'a [Edge],
    vertices: &'a [Node],
}

impl<'a> Graph<'a> {
    pub fn new(vertices: &'a [Node], edges: &'a [Edge]) -> Self {
        if !Self::validate(vertices, edges) {
            panic!()
        }

        Self { vertices, edges }
    }

    fn validate(vertices: &'a [Node], edges: &'a [Edge]) -> bool {
        let hs: HashSet<usize> = vertices.iter().map(|v| v.0).collect();
        edges.iter().all(|e| hs.contains(&e.1) && hs.contains(&e.0))
    }
}

/// Implementation of the Depth First Search algorithm
///
/// DFS(Depth First Search) uses Stack data structure.
///
/// DFS is also a traversal approach in which the traverse begins at the root
/// node and proceeds through the nodes as far as possible until we reach
/// the node with no unvisited nearby nodes.
///
/// DFS builds the tree sub-tree by sub-tree.
///
/// It works on the concept of LIFO (Last In First Out).
///
pub fn dfs(g: &Graph, root: &Node, target: &Node) -> Option<Vec<usize>> {
    let mut visited = HashSet::new();
    let mut history = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    while let Some(current) = queue.pop_front() {
        // Update the history
        history.push(current.0);

        if current == *target {
            // We've traversed to the `target` node
            return Some(history);
        }

        for neighbour in current.neighbours(g).into_iter().rev() {
            // Have we seen this Node before?
            if visited.insert(neighbour.0) {
                queue.push_front(neighbour)
            }
        }
    }

    // Unable to find path to target Node
    None
}

#[allow(unused_variables)]
/// Implementation of the Breadth First Search algorithm
///
/// BFS(Breadth First Search) uses Queue data structure for finding the shortest path.
///
/// BFS is a traversal approach in which we first walk through all nodes on the
/// same level before moving on to the next level.
///
/// BFS builds the tree level by level.
///
/// It works on the concept of FIFO (First In First Out).
pub fn bfs(g: &Graph, root: &Node, target: &Node) -> Option<Vec<usize>> {
    let mut visited = HashSet::new();
    let mut history = Vec::new();
    let mut queue = VecDeque::new();

    visited.insert(root.0);
    queue.push_back(root.clone());

    while let Some(current) = queue.pop_front() {
        history.push(current.0);

        if current == *target {
            return Some(history);
        }

        for neighbour in current.neighbours(g) {
            if !visited.contains(&neighbour.0) {
                visited.insert(neighbour.0);
                queue.push_back(neighbour.clone());
            }
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_neighbours() {
        let edges: Vec<Edge> = vec![(0, 1).into(), (1, 2).into()];
        let vertex: Vec<Node> = vec![0.into(), 1.into(), 2.into()];

        let g = Graph::new(&vertex, &edges);
        let test_vertex: Node = 0.into();

        let result = test_vertex.neighbours(&g);
        assert_eq!(1, result.len());
    }

    #[should_panic]
    #[test]
    fn test_validation() {
        let edges: Vec<Edge> = vec![(0, 1).into(), (1, 2).into()];
        let vertex: Vec<Node> = vec![1.into(), 2.into()];

        let g = Graph::new(&vertex, &edges);
        let test_vertex: Node = 0.into();

        let result = test_vertex.neighbours(&g);
        assert_eq!(1, result.len());
    }

    #[test]
    fn test_bfs() {
        let edges: Vec<Edge> = vec![(0, 1).into(), (1, 2).into()];
        let vertex: Vec<Node> = vec![0.into(), 1.into(), 2.into()];
        let g = Graph::new(&vertex, &edges);

        assert_eq!(None, bfs(&g, &2.into(), &0.into()));
        assert_eq!(3, bfs(&g, &0.into(), &2.into()).unwrap().len());
    }

    #[test]
    fn test_dfs() {
        let edges: Vec<Edge> = vec![(0, 1).into(), (1, 2).into()];
        let vertex: Vec<Node> = vec![0.into(), 1.into(), 2.into()];
        let g = Graph::new(&vertex, &edges);

        assert_eq!(None, dfs(&g, &2.into(), &0.into()));
        assert_eq!(3, dfs(&g, &0.into(), &2.into()).unwrap().len());
    }
}
