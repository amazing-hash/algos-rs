use std::collections::{BinaryHeap, VecDeque};

pub type Graph = Vec<Vec<usize>>;
pub type GraphWithWeights = Vec<Vec<(usize, u32)>>;

pub fn make_new_used(graph: &Graph) -> Vec<Color> {
    vec![Color::White; graph.len()]
}

pub fn make_path_from_parents(vertex: usize, parents: &[Option<usize>]) -> Vec<usize> {
    let mut curr = vertex;
    let mut path = vec![curr];
    while let Some(next) = parents[curr] {
        path.push(next);
        curr = next;
    }
    path.reverse();
    path
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Grey,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    Enter,
    BackEdge(usize),
    Examine(usize),
    Exit,
}

#[allow(unused)]
fn bfs<F>(graph: &Graph, from: usize, used: &mut [Color], mut cb: F)
where
    F: FnMut(usize),
{
    if used[from] == Color::White {
        let mut queue = VecDeque::new();
        queue.push_back(from);
        used[from] = Color::Black;
        cb(from);
        while let Some(from) = queue.pop_front() {
            for &to in graph[from].iter() {
                if used[to] == Color::White {
                    // visited vertex next_node
                    cb(to);
                    used[to] = Color::Black;
                    queue.push_back(to);
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn bfs_test() {
    let func = |to: usize| println!("visited node {}", to);
    let mut graph = vec![Vec::new(); 10];
    graph[1].push(2); // Add edge 1 -> 2
    graph[2].push(1); // Add edge  2 -> 1
    graph[1].push(8); // Add edge  1 -> 8
    graph[8].push(1); // Add edge  8 -> 1
    graph[2].push(3); // Add edge  2 -> 3
    graph[3].push(2); // Add edge  3 -> 2
    graph[3].push(4); // Add edge  3 -> 4
    graph[4].push(3); // Add edge  4 -> 3
    let mut used = make_new_used(&graph);
    bfs(&graph, 1, &mut used, func);
    let mut vertexes = vec![];
    let func = |to: usize| vertexes.push(to);
    let mut used = make_new_used(&graph);
    bfs(&graph, 1, &mut used, func);
    // println!("{:?}", vertexes);
}

/*
* Search for connectivity components in an undirected graph.
*/
#[allow(unused)]
fn search_connected_components<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(Vec<usize>),
{
    let mut used = make_new_used(graph);
    let n = graph.len();
    for from in 0..n {
        if used[from] == Color::White {
            let mut vertexes = vec![];
            let func = |to: usize| vertexes.push(to);
            bfs(graph, from, &mut used, func);
            cb(vertexes);
        }
    }
}

#[cfg(test)]
#[test]
fn search_connected_components_test() {
    let func = |vertexes: Vec<usize>| println!("vertexes from component {:?}", vertexes);
    let mut graph = vec![Vec::new(); 10];
    graph[1].push(2); // Add edge  1 -> 2
    graph[2].push(1); // Add edge  2 -> 1
    graph[5].push(8); // Add edge  5 -> 8
    graph[8].push(5); // Add edge  8 -> 5
    graph[5].push(9); // Add edge  5 -> 9
    graph[9].push(5); // Add edge  9 -> 5
    graph[8].push(7); // Add edge  8 -> 7
    graph[7].push(8); // Add edge  7 -> 8
    graph[2].push(3); // Add edge  2 -> 3
    graph[3].push(2); // Add edge  3 -> 2
    graph[3].push(4); // Add edge  3 -> 4
    graph[4].push(3); // Add edge  4 -> 3
    search_connected_components(&graph, func);
}

/*
* Search for strongly connectivity components in an undirected graph.
*/
#[allow(unused)]
fn search_strongly_connected_components<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(Vec<usize>),
{
    let mut graph_transp: Graph = vec![Vec::new(); graph.len()];
    #[allow(clippy::needless_range_loop)]
    for from in 0..graph.len() {
        for to in graph[from].iter() {
            graph_transp[*to].push(from);
        }
    }
    let mut visited = make_new_used(graph);
    let mut orders = Vec::with_capacity(graph.len());

    for vertex in 0..graph.len() {
        if visited[vertex] == Color::White {
            let mut func = |node: usize, event: Event| {
                if event == Event::Exit {
                    orders.push(node);
                }
            };
            dfs(graph, vertex, &mut visited, &mut func);
        }
    }
    let mut visited = make_new_used(graph);
    for vertex in orders.iter().rev() {
        if visited[*vertex] == Color::White {
            let mut vertexes = vec![];
            let mut func = |to: usize, event: Event| {
                if event == Event::Enter {
                    vertexes.push(to);
                }
            };
            dfs(&graph_transp, *vertex, &mut visited, &mut func);
            cb(vertexes);
        }
    }
}

#[cfg(test)]
#[test]
fn search_strongly_connected_components_test() {
    let func = |vertexes: Vec<usize>| println!("vertexes from component {:?}", vertexes);
    let mut graph = vec![Vec::new(); 10];
    graph[1].push(4); // Add edge  1 -> 4
    graph[4].push(7); // Add edge  4 -> 7
    graph[7].push(1); // Add edge  7 -> 1
    graph[9].push(7); // Add edge  9 -> 7
    graph[9].push(3); // Add edge  9 -> 3
    graph[3].push(6); // Add edge  3 -> 6
    graph[6].push(9); // Add edge  6 -> 9
    graph[8].push(6); // Add edge  8 -> 6
    graph[8].push(5); // Add edge  8 -> 5
    graph[5].push(2); // Add edge  5 -> 2
    graph[2].push(8); // Add edge  2 -> 8
    search_strongly_connected_components(&graph, func);
}

#[allow(unused)]
fn dfs<F>(graph: &Graph, from: usize, used: &mut [Color], cb: &mut F)
where
    F: FnMut(usize, Event),
{
    cb(from, Event::Enter);
    used[from] = Color::Grey;
    for &to in graph[from].iter() {
        if used[to] == Color::Grey {
            cb(from, Event::BackEdge(to));
        }
        if used[to] == Color::White {
            cb(to, Event::Examine(from));
            dfs(graph, to, used, cb);
        }
    }
    used[from] = Color::Black;
    cb(from, Event::Exit);
}

#[cfg(test)]
#[test]
fn dfs_test() {
    let mut func =
        |node: usize, event: Event| println!("visited vertex {}, event {:?}", node, event);
    let mut graph = vec![Vec::new(); 10];
    let mut used = make_new_used(&graph);
    graph[1].push(2); // Add edge  1 -> 2
    graph[2].push(1); // Add edge  2 -> 1
    graph[1].push(8); // Add edge  1 -> 8
    graph[8].push(1); // Add edge  8 -> 1
    graph[2].push(3); // Add edge  2 -> 3
    graph[3].push(2); // Add edge  3 -> 2
    graph[3].push(4); // Add edge  3 -> 4
    graph[4].push(3); // Add edge  4 -> 3
    dfs(&graph, 1, &mut used, &mut func);
    let mut used = make_new_used(&graph);
    let mut vertexes = vec![];
    let mut func = |node: usize, event: Event| {
        if event == Event::Enter {
            vertexes.push(node);
        }
    };
    dfs(&graph, 0, &mut used, &mut func);
    // println!("{:?}", vertexes);
}

#[allow(unused)]
fn find_cycle<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(Vec<usize>),
{
    let mut parents: Vec<Option<usize>> = vec![None; graph.len()];
    let mut used = make_new_used(graph);
    for from in 0..graph.len() {
        if used[from] == Color::White {
            let mut func = |node: usize, event: Event| {
                if let Event::Examine(from) = event {
                    parents[node] = Some(from)
                }
                if let Event::BackEdge(to) = event {
                    if parents[node] != Some(to) {
                        let mut s = node;
                        let mut vertexes = vec![];
                        vertexes.push(s);
                        while s != to {
                            s = parents[s].unwrap();
                            vertexes.push(s);
                        }
                        vertexes.reverse();
                        cb(vertexes);
                    }
                }
            };
            dfs(graph, from, &mut used, &mut func);
        }
    }
}

#[cfg(test)]
#[test]
fn find_cycle_test() {
    let func = |vertexes: Vec<usize>| println!("cycle has been detected {:?}", vertexes);
    let mut graph = vec![Vec::new(); 10];
    graph[1].push(2); // Add edge 1 -> 2
    graph[2].push(1); // Add edge 2 -> 1
    graph[1].push(8); // Add edge 1 -> 8
    graph[8].push(1); // Add edge 8 -> 1
    graph[2].push(3); // Add edge 2 -> 3
    graph[3].push(2); // Add edge 3 -> 2
    graph[3].push(4); // Add edge 3 -> 4
    graph[4].push(3); // Add edge 4 -> 3
    find_cycle(&graph, func);

    let mut graph = vec![Vec::new(); 10];
    graph[1].push(2); // Add edge 1 -> 2
    graph[2].push(1); // Add edge 2 -> 1
    graph[1].push(8); // Add edge 1 -> 8
    graph[8].push(1); // Add edge 8 -> 1
    graph[2].push(8); // Add edge 2 -> 8
    graph[8].push(2); // Add edge 8 -> 2
    graph[2].push(3); // Add edge 2 -> 3
    graph[3].push(2); // Add edge 3 -> 2
    graph[3].push(4); // Add edge 3 -> 4
    graph[4].push(3); // Add edge 4 -> 3
    graph[2].push(4); // Add edge 2 -> 4
    graph[4].push(2); // Add edge 4 -> 2
    find_cycle(&graph, func);

    let func = |vertexes: Vec<usize>| assert_eq!(vec![1, 2, 3], vertexes);
    let mut graph = vec![Vec::new(); 10];
    graph[1].push(2); // Add edge 1 -> 2
    graph[2].push(1); // Add edge 2 -> 1
    graph[1].push(3); // Add edge 1 -> 3
    graph[3].push(1); // Add edge 3 -> 1
    graph[2].push(3); // Add edge 2 -> 3
    graph[3].push(2); // Add edge 3 -> 2
    find_cycle(&graph, func);
}

#[allow(unused)]
fn find_cycle_oriented<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(Vec<usize>),
{
    let mut parents: Vec<Option<usize>> = vec![None; graph.len()];
    let mut used = make_new_used(graph);
    for from in 0..graph.len() {
        if used[from] == Color::White {
            let mut func = |node: usize, event: Event| {
                if let Event::Examine(from) = event {
                    parents[node] = Some(from)
                }
                if let Event::BackEdge(to) = event {
                    let mut s = node;
                    let mut vertexes = vec![];
                    vertexes.push(s);
                    while s != to {
                        s = parents[s].unwrap();
                        vertexes.push(s);
                    }
                    vertexes.reverse();
                    cb(vertexes);
                }
            };
            dfs(graph, from, &mut used, &mut func);
        }
    }
}

#[cfg(test)]
#[test]
fn find_cycle_oriented_test() {
    let func = |_vertexes: Vec<usize>| assert_eq!(true, false);
    let mut graph = vec![Vec::new(); 10];
    graph[1].push(2); // Add edge 1 -> 2
    graph[1].push(8); // Add edge 1 -> 8
    graph[2].push(3); // Add edge 2 -> 3
    graph[3].push(4); // Add edge 3 -> 4
    find_cycle_oriented(&graph, func);

    let func = |vertexes: Vec<usize>| assert_eq!(vertexes, vec![1, 2, 3]);
    let mut graph = vec![Vec::new(); 10];
    graph[1].push(2); // Add edge 1 -> 2
    graph[2].push(3); // Add edge 2 -> 3
    graph[3].push(1); // Add edge 3 -> 1

    find_cycle_oriented(&graph, func);
}

struct NodeDijkstra {
    node: usize,
    dist: u32,
}

impl PartialEq for NodeDijkstra {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}
impl Eq for NodeDijkstra {}
impl PartialOrd for NodeDijkstra {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for NodeDijkstra {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.partial_cmp(&self.dist).unwrap()
    }
}

#[allow(unused)]
pub fn dijkstra<F>(graph: &GraphWithWeights, from: usize, mut cb: F)
where
    F: FnMut(&[Option<u32>], &[Option<usize>]),
{
    let mut parents = vec![None; graph.len()];
    let mut visited = vec![false; graph.len()];
    let mut distances = vec![None; graph.len()];
    let mut heap = BinaryHeap::new();
    distances[from] = Some(Default::default());
    heap.push(NodeDijkstra {
        node: from,
        dist: 0u32,
    });
    while !heap.is_empty() {
        let NodeDijkstra { node, dist } = heap.pop().unwrap();
        visited[node] = true;
        for (next_node, next_weight) in &graph[node] {
            if !visited[*next_node]
                && (distances[*next_node].is_none()
                    || next_weight + dist < distances[*next_node].unwrap())
            {
                parents[*next_node] = Some(node);
                distances[*next_node] = Some(next_weight + dist);
                heap.push(NodeDijkstra {
                    node: *next_node,
                    dist: distances[*next_node].unwrap(),
                });
            }
        }
    }
    cb(&distances, &parents);
}

#[cfg(test)]
#[test]
fn dijkstra_test() {
    let func = |distances: &[Option<u32>], parents: &[Option<usize>]| {
        assert_eq!(distances[7], None);
        assert_eq!(distances[5].unwrap(), 14);
        assert_eq!(make_path_from_parents(5, parents), vec![1, 2, 3, 5]);
        assert_eq!(make_path_from_parents(3, parents), vec![1, 2, 3]);
    };
    let mut graph = vec![Vec::new(); 10];
    graph[1].push((2, 2)); // Add edge 1 -> 2
    graph[2].push((3, 5)); // Add edge 2 -> 3
    graph[3].push((5, 7)); // Add edge 3 -> 5
    graph[1].push((5, 19)); // Add edge 1 -> 5

    dijkstra(&graph, 1, func);
}

#[allow(unused)]
pub fn floid<F>(graph: &GraphWithWeights, mut cb: F)
where
    F: FnMut(&[Vec<Option<u32>>]),
{
    let mut dist = vec![vec![None; graph.len()]; graph.len()];
    for idx in 0..graph.len() {
        dist[idx][idx] = Some(0);
        for (to, weight) in graph[idx].iter() {
            dist[idx][*to] = Some(*weight);
        }
    }
    for i in 0..graph.len() {
        for j in 0..graph.len() {
            for k in 0..graph.len() {
                if dist[j][i].is_some() && dist[i][k].is_some() {
                    if dist[j][k].is_none() {
                        dist[j][k] = Some(dist[j][i].unwrap() + dist[i][k].unwrap());
                    } else {
                        dist[j][k] = Some(std::cmp::min(
                            dist[j][k].unwrap(),
                            dist[j][i].unwrap() + dist[i][k].unwrap(),
                        ));
                    }
                }
            }
        }
    }
    cb(&dist);
}

#[cfg(test)]
#[test]
fn floid_test() {
    let mut graph = vec![Vec::new(); 5];
    graph[1].push((2, 1)); // Add edge 1 -> 2
    graph[1].push((3, 6)); // Add edge 1 -> 3
    graph[2].push((3, 4)); // Add edge 2 -> 3
    graph[2].push((4, 1)); // Add edge 2 -> 4
    graph[4].push((3, 1)); // Add edge 4 -> 3

    let func = |distances: &[Vec<Option<u32>>]| {
        assert_eq!(distances[2][4].unwrap(), 1);
    };
    floid(&graph, func);
}

#[allow(unused)]
fn topological_sort<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(Vec<usize>),
{
    let mut sort = vec![];
    let mut used = make_new_used(graph);
    for from in 0..graph.len() {
        if used[from] == Color::White {
            let mut func = |node: usize, event: Event| {
                if let Event::Exit = event {
                    sort.push(node);
                }
            };
            dfs(graph, from, &mut used, &mut func);
        }
    }
    sort.reverse();
    cb(sort);
}

#[cfg(test)]
#[test]
fn topology_sort_test() {
    let mut graph = vec![Vec::new(); 10];
    graph[1].push(2); // Add edge 1 -> 2
    graph[1].push(3); // Add edge 1 -> 3
    graph[1].push(5); // Add edge 1 -> 5
    graph[1].push(4); // Add edge 1 -> 4
    graph[2].push(4); // Add edge 2 -> 4
    graph[3].push(4); // Add edge 3 -> 4
    graph[3].push(5); // Add edge 3 -> 5

    let func = |vertexes: Vec<usize>| {
        assert_eq!(vertexes, vec![9, 8, 7, 6, 1, 3, 5, 2, 4, 0]);
    };
    topological_sort(&graph, func);

    graph[6].push(7); // Add edge 6 -> 7
    graph[7].push(8); // Add edge 7 -> 8

    let func = |vertexes: Vec<usize>| {
        assert_eq!(vertexes, vec![9, 6, 7, 8, 1, 3, 5, 2, 4, 0]);
    };
    topological_sort(&graph, func);

    let mut graph = vec![Vec::new(); 10];
    graph[2].push(4); // Add edge 2 -> 4
    graph[2].push(5); // Add edge 2 -> 5
    graph[2].push(1); // Add edge 2 -> 1
    graph[3].push(1); // Add edge 3 -> 1
    graph[4].push(5); // Add edge 4 -> 5
    graph[4].push(3); // Add edge 4 -> 3
    graph[5].push(3); // Add edge 5 -> 3

    let func = |vertexes: Vec<usize>| {
        assert_eq!(vertexes, vec![9, 8, 7, 6, 2, 4, 5, 3, 1, 0]);
    };
    topological_sort(&graph, func);
}
