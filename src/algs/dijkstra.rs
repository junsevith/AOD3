use crate::algs::priority_queue::PriorityQueue;
use crate::graph::Graph;

pub fn dijkstra(graph: &Graph, start: usize) -> ShortestPaths {
    let len = graph.vertices.len();

    let mut queue = PriorityQueue::with_capacity(len);
    queue.decrease_key(start, 0);

    let mut previous = vec![None; len];

    while let Some(element) = queue.pop() {
        let new_vertex = element.id;
        let distance = element.weight;

        for edge in &graph.vertices[new_vertex].edges {
            let v = edge.destination;
            let alt = distance + edge.weight;

            if alt < queue.weight[v] {
                queue.decrease_key(v, alt);
                previous[v] = Some(new_vertex);
            }
        }

    }

    ShortestPaths {
        distance: queue.weight,
        tree: previous,
    }
}

pub fn dijkstra_pair(graph: &Graph, start: usize, end: usize) -> ShortestPath {
    let len = graph.vertices.len();

    let mut queue = PriorityQueue::with_capacity(len);
    queue.decrease_key(start, 0);

    let mut previous = vec![None; len];

    while let Some(element) = queue.pop() {
        let new_vertex = element.id;
        let distance = element.weight;

        if new_vertex == end {
            break;
        }

        for edge in &graph.vertices[new_vertex].edges {
            let v = edge.destination;
            let alt = distance + edge.weight;

            if alt < queue.weight[v] {
                queue.decrease_key(v, alt);
                previous[v] = Some(new_vertex);
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
        distance: queue.weight[end],
        path,
    }
}

#[derive(Debug)]
pub struct ShortestPaths {
    pub distance: Vec<usize>,
    pub tree: Vec<Option<usize>>,
}

#[derive(Debug)]
pub struct ShortestPath {
    pub distance: usize,
    pub path: Vec<usize>,
}

