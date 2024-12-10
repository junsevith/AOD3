use djikstra::algs::radix_heap::RadixHeap;
use rand::{rng, Rng};
use djikstra::algs::dial_bins_vec::DialBinsVec;

#[test]
fn radix_tests() {
    let mut heap = RadixHeap::new(100, 100);
    (0..100).for_each(|i| heap.add(i, rng().random_range(0..100)));
    for _ in 0..100 {
        let key = heap.next().unwrap();
        println!("{}: {}", key, heap.distance[key]);
        // println!("{:?}", heap.bins);
    }
}

#[test]
fn dial_tests() {
    let mut heap = DialBinsVec::new(100, 100);
    (0..100).for_each(|i| heap.add(i, rng().random_range(0..100)));
    while let Some(keys) = heap.next() {
        for key in keys {
            println!("{}: {}", key, heap.distance[key]);
        }
    }
}
