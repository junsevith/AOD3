pub struct DialBinsVec {
    pub distance: Vec<usize>,
    bins: Vec<Vec<usize>>,
    position: Vec<Option<(usize,usize)>>,
    cursor: usize,
    iteration: usize,
    size: usize,
}

impl DialBinsVec {
    pub fn new(c: usize, vertices: usize) -> DialBinsVec {
        DialBinsVec {
            distance: vec![usize::MAX; vertices],
            bins: vec![Vec::new(); c + 1],
            position: vec![None; vertices],
            cursor: 0,
            iteration: 0,
            size: c + 1,
        }
    }

    pub fn add(&mut self, vertex: usize, distance: usize) {
        if self.position[vertex].is_some() {
            let (bin,pos) = self.position[vertex].unwrap();
            self.bins[bin].swap_remove(pos);
            if pos < self.bins[bin].len() {
                self.position[self.bins[bin][pos]] = Some((bin,pos));
            }
        }
        self.distance[vertex] = distance;
        let bin = distance % self.size;
        self.bins[bin].push(vertex);
        self.position[vertex] = Some((bin, self.bins[bin].len() - 1));
    }

    pub fn next(&mut self) -> Option<Vec<usize>> {
        let mut count = 0;
        while count < self.size {
            if !self.bins[self.cursor].is_empty() {
                return Some(std::mem::take(&mut self.bins[self.cursor]))
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
