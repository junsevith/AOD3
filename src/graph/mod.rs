use std::collections::LinkedList;
use std::fmt::Debug;

pub struct Graph {
    pub vertices: Vec<Vertex>,
}

#[derive(Debug)]
pub struct Vertex {
    pub number: usize,
    pub edges: LinkedList<Edge>,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub destination: usize,
    pub weight: usize,
}

impl Graph {
    pub(crate) fn with_vertices(count: usize) -> Graph {
        let vertices = (0..count).map(Vertex::new).collect();
        Graph { vertices }
    }

    pub(crate) fn add_edge(&mut self, source: usize, destination: usize, weight: usize) {
        self.vertices[source]
            .edges
            .push_back(Edge::new(destination, weight));
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        if self.vertices.len() < 20 {
            for vertex in &self.vertices {
                s.push_str(&format!("Vertex {}\n", vertex.number));
                for edge in &vertex.edges {
                    s.push_str(&format!(
                        "  -> {} (weight: {})\n",
                        edge.destination, edge.weight
                    ));
                }
            }
        }

        write!(f, "{}", s)
    }
}

impl Vertex {
    fn new(number: usize) -> Vertex {
        Vertex {
            number,
            edges: LinkedList::new(),
        }
    }
}

impl Edge {
    pub fn new(destination: usize, weight: usize) -> Edge {
        Edge {
            destination,
            weight,
        }
    }
}
