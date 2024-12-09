pub struct PriorityQueue {
    pub weight: Vec<usize>,
    pub heap: Vec<usize>,
    pub position: Vec<Option<usize>>,
}

pub struct QueueElement {
    pub id: usize,
    pub weight: usize,
}

impl PriorityQueue {
    pub fn with_capacity(capacity: usize) -> PriorityQueue {
        PriorityQueue {
            weight: vec![usize::MAX; capacity],
            heap: Vec::with_capacity(capacity),
            position: vec![None; capacity],
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.heap.len()
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        self.position.swap(self.heap[i], self.heap[j]);
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    fn left(i: usize) -> usize {
        2 * i + 1
    }

    fn right(i: usize) -> usize {
        2 * i + 2
    }

    fn pos_weight(&self, position: usize) -> usize {
        self.weight[self.heap[position]]
    }

    fn heapify(&mut self, i: usize) {
        let len = self.len();
        let l = Self::left(i);
        let r = Self::right(i);

        let mut smallest = i;
        if l < len && self.pos_weight(l) < self.pos_weight(smallest) {
            smallest = l;
        }
        if r < len && self.pos_weight(r) < self.pos_weight(smallest) {
            smallest = r;
        }

        if smallest != i {
            self.swap(i, smallest);
            self.heapify(smallest);
        }
    }

    // pub fn build_heap(mut elements: Vec<QueueElement>, capacity: usize) -> PriorityQueue {
    //     elements.reserve(capacity - elements.capacity());
    //
    //     let mut res = PriorityQueue {
    //         heap: elements,
    //         position: vec![None; capacity],
    //     };
    //
    //     for i in (0..res.len() / 2).rev() {
    //         res.heapify(i);
    //     }
    //
    //     res
    // }

    fn bubble_up(&mut self, mut position: usize) {
        while position > 0 && self.pos_weight(position) < self.pos_weight(Self::parent(position)) {
            self.swap(position, Self::parent(position));
            position = Self::parent(position);
        }
    }

    pub fn push(&mut self, id: usize, weight: usize) {
        self.position[id] = Some(self.heap.len());
        self.weight[id] = weight;
        self.heap.push(id);
        self.bubble_up(self.heap.len() - 1);
    }

    pub fn pop(&mut self) -> Option<QueueElement> {

        if self.heap.is_empty() {
            return None;
        }
        self.swap(0, self.heap.len() - 1);
        let id = self.heap.pop()?;
        self.position[id] = None;
        self.heapify(0);

        Some(QueueElement {
            id,
            weight: self.weight[id],
        })
    }

    pub fn decrease_key(&mut self, id: usize, weight: usize) {
        let position = self.position[id];
        match position {
            None => {
                self.push(id, weight);
            }
            Some(position) => {
                if self.heap[position] != id {
                    panic!("Invalid decrease_key");
                }
                self.weight[id] = weight;
                self.bubble_up(position);
            }
        };

    }
}
