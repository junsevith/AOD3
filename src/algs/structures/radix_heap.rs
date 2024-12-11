use std::collections::HashSet;

pub struct RadixHeap {
    last: usize,
    pub bins: Vec<HashSet<usize>>,
    pub distance: Vec<usize>,
    position: Vec<Option<usize>>,
}

impl RadixHeap {
    pub fn new(vertices: usize, c: usize) -> RadixHeap {
        // let len = f64::log2((c + 1) as f64).ceil() + 1.;
        // let len = 64;
        let len = f64::log2((vertices * (c+1)) as f64).ceil() as usize;
        RadixHeap {
            last: 0,
            bins: vec![HashSet::new(); len as usize],
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
        self.bins[bin].insert(vertex);
        self.position[vertex] = Some(bin);
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
                    return Some(self.bins[bin].drain().next().unwrap());
                }

                let elements = self.bins[bin].drain().collect::<Vec<usize>>();
                let res = *elements.iter().min_by_key(|&x| self.distance[*x]).unwrap();
                self.last = self.distance[res];

                elements
                    .into_iter()
                    .filter(|&x| x != res)
                    .for_each(|x| self._add(x));

                return Some(res);
            }
            bin += 1;
        }
        None
    }

    pub fn decrease_key(&mut self, vertex: usize, distance: usize) {
        let bin = self.position[vertex];
        if bin.is_some() {
            self.bins[bin.unwrap()].remove(&vertex);
        }

        self.add(vertex, distance);
    }
}
