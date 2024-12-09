use djikstra::algs::dijkstra::{dijkstra, dijkstra_pair};
use djikstra::graph::Graph;

#[test]
fn test_dijkstra() {
    let data = Graph::from_file("tests/data/graph.txt");
    println!("{:?}", data);
    let res = dijkstra(&data.graph, 0);
    println!("{:?}", res);
}

#[test]
fn test_dijkstra_pairs() {
    let data = Graph::from_file("tests/data/graph.txt");
    println!("{:?}", data);
    let res = dijkstra_pair(&data.graph, 0, 5);
    println!("{:?}", res);
}

#[test]
fn test_dijkstra2() {
    let data = Graph::from_file("ch9-1.1/inputs/Long-C/Long-C.1.0.gr");
    println!("{:?}", data);
    let res = dijkstra(&data.graph, 0);
    println!("{:?}", &res.distance[0..20]);
}

#[test]
fn test_dijkstra_pairs2() {
    let data = Graph::from_file("ch9-1.1/inputs/Long-C/Long-C.1.0.gr");
    println!("{:?}", data);
    let res = dijkstra_pair(&data.graph, 0, 5);
    println!("{:?}", res.distance);
}