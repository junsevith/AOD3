use crate::algs::dial_bins::DialBins;
use crate::algs::dijkstra::ShortestPaths;
use crate::graph::Graph;

pub fn dial(graph: &Graph, start: usize, c: usize) -> ShortestPaths {
    let len = graph.vertices.len();
    let mut bins = DialBins::new(c, len);
    let mut previous = vec![None; len];
    bins.add(start, 0);

    while let Some(bin_contents) = bins.next() {
        for current in bin_contents {
            let distance = bins.distance[current];

            for edge in &graph.vertices[current].edges {
                let v = edge.destination;
                let alt = distance + edge.weight;

                if alt < bins.distance[v] {
                    bins.add(v, alt);
                    previous[v] = Some(current);
                }
            }
        }
    }

    ShortestPaths {
        distance: bins.distance,
        tree: previous,
    }
}
