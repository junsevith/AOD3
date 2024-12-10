use crate::experiments::chart::draw_chart;
use crate::experiments::experiment::{experiment, ExperimentData};
use crate::experiments::indicator::Indicator;
use crate::graph::Graph;
use rand::distr::Uniform;
use rand::{rng, Rng};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};

pub fn experiment_set(name: &str, range: RangeInclusive<usize>, path: fn(usize) -> String) {
    let iter = range;
    let mut indicator = Arc::new(Mutex::new(Indicator::new_linear(iter.clone(), name)));

    let res = iter
        .clone()
        // .into_par_iter()
        .map(move |x| {
            // println!("experiment {}", x);
            let path = path(x);
            let data = Graph::from_file(&*path);

            let range = Uniform::new(0, data.vertices).unwrap();
            let mut sources = (0..5).map(|_| rng().sample(range)).collect::<Vec<_>>();
            sources.push(0);

            let mut results = sources
                .into_iter()
                .map(|source| experiment(&data, source))
                .collect::<Vec<_>>();

            let res = results.pop().unwrap();

            let avg = ExperimentData::average(results);

            indicator.lock().unwrap().done(x);
            // println!("threads: {} current: {:?}", rayon::current_num_threads(), rayon::current_thread_index());

            (res, avg)
        })
        .collect::<Vec<(ExperimentData, ExperimentData)>>();

    let (data, averages) = ExperimentData::categorize(res);

    let names = vec!["Dijkstra", "Dial", "Radix"];

    draw_chart(data, names.clone(), iter.clone(), name, |x, y| y);

    draw_chart(
        averages,
        names,
        iter,
        &format!("{}_average", name),
        |x, y| y,
    );
}
