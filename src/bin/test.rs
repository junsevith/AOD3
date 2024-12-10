use rand::{rng, Rng};
use djikstra::algs::radix_heap::RadixHeap;

fn main() {
    let mut heap = RadixHeap::new(100, 100);
    (0..100).for_each(|i| heap.add(i, rng().random_range(0..100)));
    for _ in 0..100 {
        let key = heap.next().unwrap();
        println!("{}: {}", key, heap.distance[key]);
        println!("{:?}", heap.bins);
    }
}