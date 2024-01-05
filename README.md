# Algoritmo_BFS_RUST

# Comentários sobre o Código em Rust - BFS em Grafo

```rust
use std::collections::VecDeque;

// Função para realizar a busca em largura (BFS) em um grafo representado por uma lista de adjacências.
// Retorna a ordem dos nós visitados durante a BFS a partir do nó inicial.
fn bfs(graph: &Vec<Vec<usize>>, start: usize) -> Vec<usize> {
    // Inicialização de um vetor para rastrear os nós visitados.
    let mut visited = vec![false; graph.len()];
    // Inicialização de uma deque (fila dupla) para armazenar os nós a serem visitados.
    let mut queue = VecDeque::new();
    // Inicialização de um vetor para armazenar a ordem dos nós visitados.
    let mut order = vec![];

    // Inicialização da BFS a partir do nó inicial.
    queue.push_back(start);
    visited[start] = true;

    // Loop principal da BFS.
    while let Some(current) = queue.pop_front() {
        // Adiciona o nó atual à ordem de visita.
        order.push(current);

        // Itera sobre os vizinhos do nó atual.
        for &next in &graph[current] {
            // Se o vizinho não foi visitado, adiciona à fila e marca como visitado.
            if !visited[next] {
                queue.push_back(next);
                visited[next] = true;
            }
        }
    }

    // Retorna a ordem de visita dos nós durante a BFS.
    order
}

fn main() {
    // Representação do grafo como uma lista de adjacências.
    let graph = vec![
        vec![1, 2], // Vizinhos do nó 0
        vec![3, 4], // Vizinhos do nó 1
        vec![],     // Vizinhos do nó 2
        vec![],     // Vizinhos do nó 3
        vec![2],    // Vizinhos do nó 4
    ];

    // Nó inicial para iniciar a BFS.
    let start = 0;
    // Chama a função BFS e obtém a ordem dos nós visitados.
    let order = bfs(&graph, start);

    // Imprime a ordem dos nós visitados durante a BFS.
    println!("{:?}", order);
}




Imports: Importa a biblioteca VecDeque do módulo std::collections.
Função bfs:
graph: &Vec<Vec<usize>>, start: usize: Parâmetros da função, onde graph é a representação do grafo e start é o nó inicial para iniciar a BFS.
visited: Vetor que rastreia os nós visitados durante a BFS.
queue: Deque para armazenar os nós a serem visitados em ordem.
order: Vetor para armazenar a ordem dos nós visitados.
Loop Principal:
Enquanto há nós na fila (queue), retira o nó da frente da fila (pop_front()), adiciona à ordem de visita e visita seus vizinhos não visitados.
Função main:
Define o grafo como uma lista de adjacências.
Chama a função bfs com o nó inicial e imprime a ordem dos nós visitados.
