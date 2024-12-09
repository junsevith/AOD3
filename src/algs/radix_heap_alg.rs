use crate::algs::dijkstra::ShortestPaths;
use crate::algs::radix_heap::RadixHeap;
use crate::graph::Graph;

pub fn radix_heap(graph: &Graph, start: usize, c: usize) -> ShortestPaths {
    let len = graph.vertices.len();

    let mut heap = RadixHeap::new(len, c);
    heap.add( start, 0);

    let mut previous = vec![None; len];

    while let Some(vertex) = heap.next() {

        // println!("Vertex: {}, distance: {}", vertex, heap.distance[vertex]);

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