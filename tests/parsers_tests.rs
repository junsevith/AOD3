use djikstra::graph::Graph;

#[test]
fn test_generate_graph() {
    let data = Graph::from_file("tests/data/graph.txt");
    println!("{:?}", data);
}



