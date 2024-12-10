use djikstra::experiments::experiment_set::experiment_set;
use std::ops::RangeInclusive;
use rayon::prelude::IntoParallelIterator;
use rayon::iter::ParallelIterator;

fn main() {
    let usa_types = vec![
        "BAY", "CAL", "COL", "CTR", "E", "FLA", "LKS", "NE", "NW", "NY", "W"
    ];
    let usa_range = 0..=(usa_types.len()-1);

    let inputs:Vec<(&str, RangeInclusive<usize>, fn(usize) ->String)> = vec![
        // ("Random4-n", 10..=21, |x| format!("ch9-1.1/inputs/Random4-n/Random4-n.{}.0.gr", x)), //success
        // ("Random4-c", 0..=15, |x| format!("ch9-1.1/inputs/Random4-c/Random4-c.{}.0.gr", x)), //success
        // ("Long-n", 10..=21, |x| format!("ch9-1.1/inputs/Long-n/Long-n.{}.0.gr", x)), //success
        // ("Square-n", 10..=21, |x| format!("ch9-1.1/inputs/Square-n/Square-n.{}.0.gr", x)), //success
        // ("Long-C", 0..=13, |x| format!("ch9-1.1/inputs/Long-C/Long-C.{}.0.gr", x)),
        ("Square-C", 0..=14, |x| format!("ch9-1.1/inputs/Square-C/Square-C.{}.0.gr", x)),
        ("USA-road-t", usa_range.clone(), |x| format!("ch9-1.1/inputs/USA-road-t/USA-road-t.{}.gr", usa_type(x))), //success
        ("USA_road-d", usa_range.clone(), |x| format!("ch9-1.1/inputs/USA-road-d/USA-road-d.{}.gr", usa_type(x))) //success
    ];

    inputs.into_iter().for_each(|(name, range, path)| {
        println!("Running experiment set: {}", name);
        experiment_set(name, range, path);
    });



    // experiment_set("Long-C", 0..=6, |x| format!("ch9-1.1/inputs/Long-C/Long-C.{}.0.gr", x));
}

fn usa_type(x: usize) -> String {
    let usa_types = vec![
        "BAY", "CAL", "COL", "CTR", "E", "FLA", "LKS", "NE", "NW", "NY", "W"
    ];
    usa_types[x].to_string()
}
