use djikstra::algs::dial::dial;
use djikstra::algs::radix_heap_alg::radix_heap;
use djikstra::graph::Graph;

#[test]
fn radix_test() {
    let data = Graph::from_file("ch9-1.1/inputs/Long-C/Long-C.1.0.gr");
    println!("{:?}", data);
    let res = radix_heap(&data.graph, 0, data.max_weight);
    println!("{:?}", &res.distance[0..20]);
}