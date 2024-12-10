use djikstra::algs::dial::dial;
use djikstra::algs::radix_heap_alg::radix_heap;
use djikstra::graph::Graph;

fn main() {
    // let data = Graph::from_file("ch9-1.1/inputs/Random4-c/Random4-c.0.0.gr");
    let data = Graph::from_file("ch9-1.1/inputs/Square-C/Square-C.14.0.gr");
    // let data = Graph::from_file("ch9-1.1/inputs/Long-C/Long-C.13.0.gr");
    println!("{:?}", data);
    let res = dial(&data.graph, 0, data.max_weight);
    println!("{:?}", &res.distance[0..20]);
}