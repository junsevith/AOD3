use djikstra::algs::radix_heap_alg::{radix_heap, radix_heap_pair};
use djikstra::parsers::input::{parse_input, ExperimentType};
use std::io::Write;

fn main() {
    let (data, input, mut output) = parse_input();
    write!(output, "c Algorytm Diala\n").unwrap();
    match input {
        ExperimentType::Ss(sources) => {
            for source in sources {
                println!("Experimenting with source {}", source);
                let start = std::time::Instant::now();
                let _ = radix_heap(&data.graph, source, data.max_weight);
                let dijkstra_time = start.elapsed().as_nanos();
                let milis = dijkstra_time as f64 / 1_000_000.0;
                write!(output, "t {}\n", milis).unwrap();
            }
        }
        ExperimentType::P2p(pairs) => {
            for (source, target) in pairs {
                println!("Experimenting with pair {} {}", source, target);
                let res = radix_heap_pair(&data.graph, source, target, data.max_weight);
                write!(output, "d {} {} {}\n", source, target, res.distance).unwrap();
            }
        }
    }
}