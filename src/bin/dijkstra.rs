use djikstra::algs::dijkstra::{dijkstra, dijkstra_pair};
use djikstra::parsers::input::{parse_input, ExperimentType};
use std::io::Write;

fn main() {
    let (data, input, mut output) = parse_input();
    write!(output, "c Algorytm Dijkstry\n").unwrap();
    match input {
        ExperimentType::Ss(sources) => {
            for source in sources {
                println!("Experimenting with source {}", source);
                let start = std::time::Instant::now();
                let _ = dijkstra(&data.graph, source);
                let dijkstra_time = start.elapsed().as_nanos();
                let milis = dijkstra_time as f64 / 1_000_000.0;
                write!(output, "t {}\n", milis).unwrap();
            }
        }
        ExperimentType::P2p(pairs) => {
            for (source, target) in pairs {
                println!("Experimenting with pair {} {}", source, target);
                let res = dijkstra_pair(&data.graph, source, target);
                write!(output, "d {} {} {}\n", source, target, res.distance).unwrap();
            }
        }
    }

}