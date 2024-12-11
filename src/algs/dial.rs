use crate::algs::dijkstra::{ShortestPath, ShortestPaths};
use crate::algs::structures::dial_bins_vec::DialBinsVec;
use crate::graph::Graph;

pub fn dial(graph: &Graph, start: usize, c: usize) -> ShortestPaths {

    // if graph.vertices.len() > 2usize.pow(19) {
    //     return ShortestPaths {
    //         distance: vec![usize::MAX; graph.vertices.len()],
    //         tree: vec![None; graph.vertices.len()],
    //     }
    // }

    let len = graph.vertices.len();
    let mut bins = DialBinsVec::new(c, len);
    let mut previous = vec![None; len];
    bins.add(start, 0);

    // let mut count = 0;

    while let Some(bin_contents) = bins.next() {

        // count += 1;
        // if count % (len / 100) == 0 {
        //     println!("Done: {}%", count*100 / len);
        // }

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

pub fn dial_pair(graph: &Graph, start: usize, end: usize, c: usize) -> ShortestPath {

    let len = graph.vertices.len();
    let mut bins = DialBinsVec::new(c, len);
    let mut previous = vec![None; len];
    bins.add(start, 0);

    // let mut count = 0;

    while let Some(bin_contents) = bins.next() {
        for current in bin_contents {

            if current == end {
                break;
            }

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

    let mut path = vec![];
    let mut current = end;
    while let Some(prev) = previous[current] {
        path.push(current);
        current = prev;
    }
    path.push(start);
    path.reverse();

    ShortestPath {
        distance: bins.distance[end],
        path,
    }
}