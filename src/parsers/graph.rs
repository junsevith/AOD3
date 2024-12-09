use std::fmt::Debug;
use crate::graph::Graph;
use std::fs::File;
use std::io::{BufRead, BufReader};

impl Graph {
    pub fn from_file(path: &str) -> GraphData {
        let file = File::open(path).unwrap();
        let buf_reader = BufReader::new(file);

        let mut graph = Graph { vertices: vec![] };

        let mut declared_edges = 0;
        let mut found_edges = 0;

        let mut min_weight = 0;
        let mut max_weight = 0;

        let mut p_type = String::new();

        for line in buf_reader.lines() {
            let line = line.unwrap();
            match line.chars().next().unwrap() {
                'c' => continue,
                'p' => {
                    let mut iter = line.split_whitespace();
                    let _ = iter.next();
                    p_type = iter.next().unwrap().to_string();
                    let vertices = iter.next().unwrap().parse::<usize>().unwrap();
                    declared_edges = iter.next().unwrap().parse::<usize>().unwrap();
                    graph = Graph::with_vertices(vertices);
                    // print!(
                    //     "Parsing graph with {} vertices and {} edges ... ",
                    //     vertices, declared_edges
                    // );
                }
                'a' => {
                    let mut iter = line.split_whitespace();
                    let _ = iter.next();
                    let source = iter.next().unwrap().parse::<usize>().unwrap();
                    let destination = iter.next().unwrap().parse::<usize>().unwrap();
                    let weight = iter.next().unwrap().parse::<usize>().unwrap();
                    graph.add_edge(source - 1, destination - 1, weight);

                    if weight < min_weight {
                        min_weight = weight;
                    }
                    if weight > max_weight {
                        max_weight = weight;
                    }

                    found_edges += 1;
                }
                _ => continue,
            }
        }

        if declared_edges != found_edges {
            panic!("Edges count does not match the number of edges in the file");
        }
        // println!("Parsing done");

        let vertices = graph.vertices.len();
        GraphData {
            graph,
            vertices,
            edges: found_edges,
            min_weight,
            max_weight,
        }
    }
}

pub struct GraphData {
    pub graph: Graph,
    pub vertices: usize,
    pub edges: usize,
    pub min_weight: usize,
    pub max_weight: usize,
}

impl Debug for GraphData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Graph with {} vertices and {} edges\n\
            Minimum weight: {}\n\
            Maximum weight: {}\n\
            {:?}",
            self.vertices, self.edges, self.min_weight, self.max_weight, self.graph
        )
    }
}
