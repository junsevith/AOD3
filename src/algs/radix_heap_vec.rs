pub struct RadixHeapVec {
    last: usize,
    pub bins: Vec<Vec<usize>>,
    pub distance: Vec<usize>,
    position: Vec<Option<(usize,usize)>>,
}

impl RadixHeapVec {
    pub fn new(vertices: usize, c: usize) -> RadixHeapVec {
        // let len = f64::log2((c + 1) as f64).ceil() + 1.;
        // let len = 64;
        let len = f64::log2((vertices * (c+1)) as f64).ceil() as usize;
        RadixHeapVec {
            last: 0,
            bins: vec![Vec::new(); len],
            distance: vec![usize::MAX; vertices],
            position: vec![None; vertices],
        }
    }

    fn get_bin(&self, distance: usize) -> usize {
        // if distance - self.last > 4 {
        //     panic!("Distance too large");
        // }

        let mut bin = 0;
        let mut d = distance ^ self.last;
        while d > 0 {
            d >>= 1;
            bin += 1;
        }
        bin
    }

    pub fn add(&mut self, vertex: usize, distance: usize) {
        self.distance[vertex] = distance;
        self._add(vertex);
    }

    fn _add(&mut self, vertex: usize) {
        let distance = self.distance[vertex];
        let bin = self.get_bin(distance);
        self.bins[bin].push(vertex);
        self.position[vertex] = Some((bin, self.bins[bin].len()-1));
    }

    pub fn next(&mut self) -> Option<usize> {
        // println!("Last: {}", self.last);
        // self.bins.iter().for_each(|x| {
        //     println!(
        //         "{:?}",
        //         x.iter().map(|x| self.distance[*x]).collect::<Vec<usize>>()
        //     )
        // });

        let mut bin = 0;
        while bin < self.bins.len() {
            if !self.bins[bin].is_empty() {

                if bin == 0 || bin == 1 {
                    return self.bins[bin].pop()
                }

                let len = self.bins[bin].len();
                let mut bin = std::mem::replace(&mut self.bins[bin], Vec::with_capacity(len));
                let min = *bin.iter().min_by_key(|&x| self.distance[*x]).unwrap();
                self.last = self.distance[min];
                self.position[min] = None;

                bin.into_iter().filter(|&x| x != min).for_each(|x| self._add(x));

                return Some(min);
            }
            bin += 1;
        }
        None
    }

    pub fn decrease_key(&mut self, vertex: usize, distance: usize) {
        let pos = self.position[vertex];

        if let Some(pos) = pos {
            let (bin, pos) = pos;
            self.bins[bin].swap_remove(pos);
            if pos != self.bins[bin].len() {
                let other = self.bins[bin][pos];
                self.position[other] = Some((bin, pos));
            }
        }

        self.add(vertex, distance);
    }
}
