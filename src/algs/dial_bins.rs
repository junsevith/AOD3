use std::collections::{HashMap, HashSet, LinkedList};

pub struct DialBins {
    pub distance: Vec<usize>,
    pub bins: Vec<HashSet<usize>>,
    cursor: usize,
    iteration: usize,
    size: usize,
}

impl DialBins {
    pub fn new(c: usize, vertices: usize) -> DialBins {
        DialBins {
            distance: vec![usize::MAX; vertices],
            bins: vec![HashSet::new(); c + 1],
            cursor: 0,
            iteration: 0,
            size: c + 1,
        }
    }

    pub fn add(&mut self, vertex: usize, distance: usize) {
        if self.distance[vertex] != usize::MAX {
            let old = self.distance[vertex] % self.size;
            self.bins[old].remove(&vertex);
        }
        self.distance[vertex] = distance;
        self.bins[distance % self.size].insert(vertex);
    }

    pub fn next(&mut self) -> Option<Vec<usize>> {
        let mut count = 0;
        while count < self.size {
            // let bin = &mut self.bins[self.cursor];
            if !self.bins[self.cursor].is_empty() {
                // let res = bin.iter().next().unwrap().clone();
                // bin.remove(&res);
                // return Some(res);
                return Some(self.bins[self.cursor].drain().collect())
            }
            self.increment_cursor();
            count += 1;
        }

        None
    }

    fn increment_cursor(&mut self) {
        self.cursor += 1;
        if self.cursor == self.size {
            self.cursor = 0;
            self.iteration += 1;
        }
    }
}
