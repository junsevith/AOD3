use std::ops::RangeInclusive;
use crate::experiments::experiment::{experiment, ExperimentData};
use crate::experiments::indicator::Indicator;
use crate::graph::Graph;
use rand::distr::Uniform;
use rand::{rng, Rng};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};
use crate::experiments::chart::draw_chart;

pub fn experiment_set(name: &str, range: RangeInclusive<usize>, path: fn(usize) ->String) {
    let iter = range;
    let mut indicator = Arc::new(Mutex::new(Indicator::new_linear(iter.clone(), name)));

    let res = iter.clone()
        .into_par_iter()
        .map(move |x| {
            // println!("experiment {}", x);
            let path = path(x);
            let data = Graph::from_file(&*path);

            let res = experiment(&data, 0);

            let range = Uniform::new(0, data.vertices).unwrap();
            let avg_data = (0..5)
                .map(|_| {
                    let source = rng().sample(range);
                    experiment(&data, source)
                })
                .collect();

            let avg = ExperimentData::average(avg_data);

            indicator.lock().unwrap().done(x);
            // println!("threads: {} current: {:?}", rayon::current_num_threads(), rayon::current_thread_index());

            (res, avg)
        })
        .collect::<Vec<(ExperimentData, ExperimentData)>>();

    let (data, averages) = ExperimentData::categorize(res);

    let names = vec!["Dijkstra", "Dial", "Radix"];

    draw_chart(data, names.clone(), iter.clone(), name, |x, y| y);

    draw_chart(averages, names, iter, &format!("{}_average", name), |x, y| y);
}
