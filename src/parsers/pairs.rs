use std::io::BufRead;

pub fn parse_pairs(path: &str) -> Vec<(usize, usize)> {
    let file = std::fs::File::open(path).unwrap();
    let buf_reader = std::io::BufReader::new(file);

    let mut pairs = vec![];

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
            'q' => {
                let mut iter = line.split_whitespace();
                let _ = iter.next();
                let first = iter.next().unwrap().parse::<usize>().unwrap();
                let second = iter.next().unwrap().parse::<usize>().unwrap();
                pairs.push((first-1, second-1));
                count -= 1;
            },
            _ => continue,
        }
    }

    if count != 0 {
        panic!("Pairs count does not match the number of pairs in the file");
    }

    pairs
}