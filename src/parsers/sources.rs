use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_sources(path: &str) -> Vec<usize> {
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);

    let mut starts = vec![];

    let mut count = 0;

    for line in buf_reader.lines() {
        let line = line.unwrap();
        match line.chars().next().unwrap() {
            'c' => continue,
            'p' => {
                let mut iter = line.split_whitespace();
                let _ = iter.next();
                let _ = iter.next();
                let _ = iter.next();
                let _ = iter.next();
                count = iter.next().unwrap().parse::<usize>().unwrap();
            },
            's' => {
                let mut iter = line.split_whitespace();
                let _ = iter.next();
                let start = iter.next().unwrap().parse::<usize>().unwrap();
                starts.push(start-1);
                count -= 1;
            },
            _ => continue,
        }
    }

    if count != 0 {
        panic!("Starts count does not match the number of starts in the file");
    }

    starts
}