use rand::distr::Uniform;
use rand::{rng, Rng};
use djikstra::algs::dial::dial_pair;
use djikstra::algs::dijkstra::dijkstra_pair;
use djikstra::algs::radix_heap_alg::radix_heap_pair;
use djikstra::graph::Graph;

fn main() {
    let inputs = vec![
        ("Random4-n-21", "ch9-1.1/inputs/Random4-n/Random4-n.21.0.gr"), //success
        ("Random4-c-15", "ch9-1.1/inputs/Random4-c/Random4-c.15.0.gr"), //success
        ("Long-n-21", "ch9-1.1/inputs/Long-n/Long-n.21.0.gr"), //success
        ("Square-n-21","ch9-1.1/inputs/Square-n/Square-n.21.0.gr"), //success
        ("USA-road-t-CTR", "ch9-1.1/inputs/USA-road-t/USA-road-t.CTR.gr"), //success
        ("USA_road-d-CTR", "ch9-1.1/inputs/USA-road-d/USA-road-d.CTR.gr"), //success
        ("Long-C-13","ch9-1.1/inputs/Long-C/Long-C.13.0.gr"), //success
        ("Square-C-14", "ch9-1.1/inputs/Square-C/Square-C.14.0.gr"),
    ];

    inputs.into_iter().for_each(|(name, path)| {
        experiment_set(name, path);
    });
}

fn experiment_set(name: &str, path: &str) {
    println!("Running experiment set: {}", name);
    let data = Graph::from_file(path);
    let range = Uniform::new(0, data.vertices).unwrap();
    let mut pairs = (0..5).map(|_| (rng().sample(range), rng().sample(range))).collect::<Vec<_>>();
    pairs.push((0,data.vertices-1));

    pairs.into_iter().for_each(|(start, end)| {
        experiment(&data.graph, start, end, data.max_weight);
    });
}

fn experiment(graph: &Graph, start: usize, end: usize, c: usize) {
    let res_dial = dial_pair(&graph, start, end, c);
    let res_radix = radix_heap_pair(&graph, start, end, c);
    let res_dijkstra = dijkstra_pair(&graph, start, end);

    println!("$ {} $,$ {} $,$ {} $,$ {} $,$ {} $,", start, end, res_dijkstra.distance, res_dial.distance, res_radix.distance);
}