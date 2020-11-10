mod graph;
use graph::Graph;

fn main() {
    // Load the graph.
    let graph = Graph::load_from("/home/jaytee/graph.txt")
        .unwrap_or_else(|err| panic!("Failed to load graph from file: {}", err));

    // Calculate its shortest-path matrix.
}
