use std::collections::{BinaryHeap, VecDeque};

pub type Graph = Vec<Vec<usize>>;
pub type GraphWithWeights = Vec<Vec<(usize, u32)>>;

pub fn make_new_used(graph: &Graph) -> Vec<bool> {
    vec![false; graph.len()]
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
    Exit,
}

#[allow(unused)]
fn bfs<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(usize),
{
    let mut used = make_new_used(graph);
    let n = graph.len();
    for start_node in 0..n {
        if !used[start_node] {
            // visited vertex start_node
            cb(start_node);
            let mut queue = VecDeque::new();
            queue.push_back(start_node);
            used[start_node] = true;
            while let Some(curr_node) = queue.pop_front() {
                for &next_node in graph[curr_node].iter() {
                    if !used[next_node] {
                        // visited vertex next_node
                        cb(next_node);
                        used[next_node] = true;
                        queue.push_back(next_node);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn bfs_test() {
    let func = |node: usize| println!("visited vertex {}", node);
    let mut graph = vec![Vec::new(); 10];
    graph[1].push(2); // Add edge 1 -> 2
    graph[2].push(1); // Add edge  2 -> 1
    graph[1].push(8); // Add edge  1 -> 8
    graph[8].push(1); // Add edge  8 -> 1
    graph[2].push(3); // Add edge  2 -> 3
    graph[3].push(2); // Add edge  3 -> 2
    graph[3].push(4); // Add edge  3 -> 4
    graph[4].push(3); // Add edge  4 -> 3
    bfs(&graph, func);
    let mut vertexes = vec![];
    let func = |node: usize| vertexes.push(node);
    bfs(&graph, func);
    // println!("{:?}", vertexes);
}

/*
* Search for connectivity components in an undirected graph.
*/
#[allow(unused)]
fn search_connected_components<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(usize, i32),
{
    let mut used = make_new_used(graph);
    let n = graph.len();
    let mut comp_id = 1;
    for start_node in 0..n {
        if !used[start_node] {
            // The beginning of the traversal of the connectivity component
            // Visited vertex start_node from comp_id;
            cb(start_node, comp_id);
            let mut queue = VecDeque::new();
            queue.push_back(start_node);
            used[start_node] = true;
            while let Some(curr_node) = queue.pop_front() {
                for &next_node in graph[curr_node].iter() {
                    if !used[next_node] {
                        // Visited vertex next_node from comp_id;
                        cb(next_node, comp_id);
                        used[next_node] = true;
                        queue.push_back(next_node);
                    }
                }
            }
            // Completing the traversal of the connectivity component
            comp_id += 1;
        }
    }
}

#[cfg(test)]
#[test]
fn search_connected_components_test() {
    use std::collections::HashMap;
    let func =
        |node: usize, comp_id: i32| println!("visited vertex {} from component {}", node, comp_id);
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
    let mut components = HashMap::new();
    let func = |node: usize, comp_id| {
        let entry = components.entry(comp_id).or_insert(vec![]);
        entry.push(node);
    };
    search_connected_components(&graph, func);
    // println!("{:?}", components);
}

/*
* Search for strongly connectivity components in an undirected graph.
*/
#[allow(unused)]
fn search_strongly_connected_components<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(usize, i32),
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
        if !visited[vertex] {
            let func = |node: usize, event: Event| {
                if event == Event::Exit {
                    orders.push(node);
                }
            };
            dfs(graph, &mut visited, func);
        }
    }
    let mut visited = make_new_used(graph);
    let mut comp_id = 1;
    for vertex in orders.iter().rev() {
        if !visited[*vertex] {
            let func = |node: usize, event: Event| {
                if event == Event::Enter {
                    cb(node, comp_id);
                }
            };
            dfs_from(&graph_transp, *vertex, &mut visited, func);
            comp_id += 1;
        }
    }
}

#[cfg(test)]
#[test]
fn search_strongly_connected_components_test() {
    let func = |node: usize, comp_id: i32| {
        println!("!!! visited vertex {} from component {}", node, comp_id)
    };
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
fn dfs<F>(graph: &Graph, used: &mut [bool], mut cb: F)
where
    F: FnMut(usize, Event),
{
    let n = graph.len();
    for start_node in 0..n {
        if !used[start_node] {
            dfs_innner(graph, used, start_node, &mut cb);
        }
    }
}

#[allow(unused)]
fn dfs_from<F>(graph: &Graph, start_node: usize, used: &mut [bool], mut cb: F)
where
    F: FnMut(usize, Event),
{
    dfs_innner(graph, used, start_node, &mut cb);
}

#[allow(unused)]
fn dfs_innner<F>(graph: &Graph, used: &mut [bool], curr_node: usize, cb: &mut F)
where
    F: FnMut(usize, Event),
{
    cb(curr_node, Event::Enter);
    used[curr_node] = true;
    for &next_node in graph[curr_node].iter() {
        if !used[next_node] {
            dfs_innner(graph, used, next_node, cb);
        }
    }
    cb(curr_node, Event::Exit);
}

#[cfg(test)]
#[test]
fn dfs_test() {
    let func = |node: usize, event: Event| println!("visited vertex {}, event {:?}", node, event);
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
    dfs(&graph, &mut used, func);
    let mut used = make_new_used(&graph);
    let mut vertexes = vec![];
    let func = |node: usize, event: Event| {
        if event == Event::Enter {
            vertexes.push(node);
        }
    };
    dfs(&graph, &mut used, func);
    // println!("{:?}", vertexes);
}

#[allow(unused)]
fn find_cycle<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(Vec<usize>),
{
    let mut parents: Vec<Option<usize>> = vec![None; graph.len()];
    let mut colors = vec![Color::White; graph.len()];
    let n = graph.len();
    for start_node in 0..n {
        if colors[start_node] == Color::White {
            find_cycle_inner(graph, &mut colors, &mut parents, start_node, &mut cb);
        }
    }
}

#[allow(unused)]
fn find_cycle_inner<F>(
    graph: &Graph,
    colors: &mut [Color],
    parents: &mut [Option<usize>],
    curr_node: usize,
    cb: &mut F,
) where
    F: FnMut(Vec<usize>),
{
    colors[curr_node] = Color::Grey;
    for &next_node in graph[curr_node].iter() {
        if colors[next_node] == Color::Grey && Some(next_node) != parents[curr_node] {
            // cycle has been detected
            let mut s = curr_node;
            let mut vertexes = vec![];
            vertexes.push(s);
            while s != next_node {
                s = parents[s].unwrap();
                vertexes.push(s);
            }
            vertexes.reverse();
            cb(vertexes);
        }
        if colors[next_node] == Color::White {
            parents[next_node] = Some(curr_node);
            find_cycle_inner(graph, colors, parents, next_node, cb);
        }
    }
    colors[curr_node] = Color::Black;
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
    let mut colors = vec![Color::White; graph.len()];
    let n = graph.len();
    for start_node in 0..n {
        if colors[start_node] == Color::White {
            find_cycle_oriented_inner(graph, &mut colors, &mut parents, start_node, &mut cb);
        }
    }
}

#[allow(unused)]
fn find_cycle_oriented_inner<F>(
    graph: &Graph,
    colors: &mut [Color],
    parents: &mut [Option<usize>],
    curr_node: usize,
    cb: &mut F,
) where
    F: FnMut(Vec<usize>),
{
    colors[curr_node] = Color::Grey;
    for &next_node in graph[curr_node].iter() {
        if colors[next_node] == Color::Grey {
            // cycle has been detected
            let mut s = curr_node;
            let mut vertexes = vec![];
            vertexes.push(s);
            while s != next_node {
                s = parents[s].unwrap();
                vertexes.push(s);
            }
            vertexes.reverse();
            cb(vertexes);
        }
        if colors[next_node] == Color::White {
            parents[next_node] = Some(curr_node);
            find_cycle_oriented_inner(graph, colors, parents, next_node, cb);
        }
    }
    colors[curr_node] = Color::Black;
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
