use djikstra::algs::dial::dial;
use djikstra::algs::dijkstra::dijkstra;
use djikstra::algs::radix_heap_alg::radix_heap;
use djikstra::parsers::graph::GraphData;

pub fn experiment(data: &GraphData, source: usize) -> ExperimentData {
    let dijkstra_time = {
        let start = std::time::Instant::now();
        let _ = dijkstra(&data.graph, source);
        start.elapsed().as_nanos()
    } as f64;

    let dial_time = {
        let start = std::time::Instant::now();
        let _ = dial(&data.graph, source, data.max_weight);
        start.elapsed().as_nanos()
    } as f64;

    let radix_time = {
        let start = std::time::Instant::now();
        let _ = radix_heap(&data.graph, source, data.max_weight);
        start.elapsed().as_nanos()
    } as f64;

    ExperimentData {
        dijkstra_time,
        dial_time,
        radix_time,
    }
}

pub struct ExperimentData {
    pub dijkstra_time: f64,
    pub dial_time: f64,
    pub radix_time: f64,
}

impl ExperimentData {
    pub fn average(data: Vec<ExperimentData>) -> ExperimentData {
        let mut dijkstra_avg = 0.;
        let mut dial_avg = 0.;
        let mut radix_avg = 0.;

        let len = data.len() as f64;

        for d in data {
            dijkstra_avg += d.dijkstra_time;
            dial_avg += d.dial_time;
            radix_avg += d.radix_time;
        }

        dijkstra_avg /= len;
        dial_avg /= len;
        radix_avg /= len;

        ExperimentData {
            dijkstra_time: dijkstra_avg,
            dial_time: dial_avg,
            radix_time: radix_avg,
        }
    }

    pub fn categorize(data: Vec<(ExperimentData, ExperimentData)>) -> (Vec<Vec<f64>>,Vec<Vec<f64>>) {
        let len = data.len();

        let mut dijkstra = Vec::with_capacity(len);
        let mut dial = Vec::with_capacity(len);
        let mut radix = Vec::with_capacity(len);

        let mut dijkstra_avg = Vec::with_capacity(len);
        let mut dial_avg = Vec::with_capacity(len);
        let mut radix_avg = Vec::with_capacity(len);

        for (d, a) in data {
            dijkstra.push(d.dijkstra_time);
            dial.push(d.dial_time);
            radix.push(d.radix_time);

            dijkstra_avg.push(a.dijkstra_time);
            dial_avg.push(a.dial_time);
            radix_avg.push(a.radix_time);
        }

        (vec![dijkstra, dial, radix], vec![dijkstra_avg, dial_avg, radix_avg])
    }
}