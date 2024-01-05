use std::collections::VecDeque;

fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();
    let mut order = vec![];

    queue.push_back(start);
    visited[start] = true;

    while let Some(current) = queue.pop_front() {
        order.push(current);

        for &next in &graph[current] {
            if !visited[next] {
                queue.push_back(next);
                visited[next] = true;
            }
        }
    }

    order
}

fn main() {
    let graph = vec![
        vec![1, 2], // vizinhos do nó 0
        vec![3, 4], // vizinhos do nó 1
        vec![],     // vizinhos do nó 2
        vec![],     // vizinhos do nó 3
        vec![2],    // vizinhos do nó 4
    ];

    let start = 0;
    let order = bfs(&graph, start);

    println!("{:?}", order);
}