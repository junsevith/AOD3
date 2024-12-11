use crate::algs::dijkstra::{ShortestPath, ShortestPaths};
use crate::algs::structures::radix_heap_vec::RadixHeapVec;
use crate::graph::Graph;

pub fn radix_heap(graph: &Graph, start: usize, c: usize) -> ShortestPaths {
    let len = graph.vertices.len();

    let mut heap = RadixHeapVec::new(len, c);
    heap.add(start, 0);

    let mut previous = vec![None; len];

    // let mut count = 0;

    while let Some(vertex) = heap.next() {
        // count += 1;
        // if count % (len / 100) == 0 {
        //     println!("Done: {}%", count*100 / len);
        // }

        let distance = heap.distance[vertex];
        for edge in &graph.vertices[vertex].edges {
            let v = edge.destination;
            let alt = distance + edge.weight;

            if alt < heap.distance[v] {
                heap.decrease_key(v, alt);
                previous[v] = Some(vertex);
            }
        }
    }

    ShortestPaths {
        distance: heap.distance,
        tree: previous,
    }
}

pub fn radix_heap_pair(graph: &Graph, start: usize, end:usize, c: usize) -> ShortestPath {
    let len = graph.vertices.len();

    let mut heap = RadixHeapVec::new(len, c);
    heap.add(start, 0);

    let mut previous = vec![None; len];

    while let Some(vertex) = heap.next() {

        if vertex == end {
            break;
        }

        let distance = heap.distance[vertex];
        for edge in &graph.vertices[vertex].edges {
            let v = edge.destination;
            let alt = distance + edge.weight;

            if alt < heap.distance[v] {
                heap.decrease_key(v, alt);
                previous[v] = Some(vertex);
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
        distance: heap.distance[end],
        path,
    }
}
