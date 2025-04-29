use std::collections::VecDeque;

pub type Graph = Vec<Vec<usize>>;

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
    F: FnMut(usize) -> (),
{
    let mut used = vec![false; graph.len()];
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
fn search_components<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(usize, i32) -> (),
{
    let mut used = vec![false; graph.len()];
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
fn search_components_test() {
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
    search_components(&graph, func);
    let mut components = HashMap::new();
    let func = |node: usize, comp_id| {
        let entry = components.entry(comp_id).or_insert(vec![]);
        entry.push(node);
    };
    search_components(&graph, func);
    // println!("{:?}", components);
}

#[allow(unused)]
fn dfs<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(usize, Event) -> (),
{
    let mut used = vec![false; graph.len()];
    let n = graph.len();
    for start_node in 0..n {
        if !used[start_node] {
            dfs_innner(graph, &mut used, start_node, &mut cb);
        }
    }
}

#[allow(unused)]
fn dfs_innner<F>(graph: &Graph, used: &mut [bool], curr_node: usize, cb: &mut F)
where
    F: FnMut(usize, Event) -> (),
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

#[test]
fn dfs_test() {
    let func = |node: usize, event: Event| println!("visited vertex {}, event {:?}", node, event);
    let mut graph = vec![Vec::new(); 10];
    graph[1].push(2); // Add edge  1 -> 2
    graph[2].push(1); // Add edge  2 -> 1
    graph[1].push(8); // Add edge  1 -> 8
    graph[8].push(1); // Add edge  8 -> 1
    graph[2].push(3); // Add edge  2 -> 3
    graph[3].push(2); // Add edge  3 -> 2
    graph[3].push(4); // Add edge  3 -> 4
    graph[4].push(3); // Add edge  4 -> 3
    dfs(&graph, func);
    let mut vertexes = vec![];
    let func = |node: usize, event: Event| {
        if event == Event::Enter {
            vertexes.push(node);
        }
    };
    dfs(&graph, func);
    // println!("{:?}", vertexes);
}

#[allow(unused)]
fn find_cycle<F>(graph: &Graph, mut cb: F)
where
    F: FnMut(Vec<usize>) -> (),
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
    F: FnMut(Vec<usize>) -> (),
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
