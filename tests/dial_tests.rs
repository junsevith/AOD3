use djikstra::algs::dial::dial;
use djikstra::algs::dijkstra::dijkstra;
use djikstra::graph::Graph;

#[test]
fn test_dial() {
    let data = Graph::from_file("ch9-1.1/inputs/Long-C/Long-C.1.0.gr");
    println!("{:?}", data);
    let res = dial(&data.graph, 0, data.max_weight);
    println!("{:?}", &res.distance[0..20]);
}