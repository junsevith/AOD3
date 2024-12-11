use crate::graph::Graph;
use crate::parsers::graph::GraphData;
use crate::parsers::pairs::parse_pairs;
use crate::parsers::sources::parse_sources;
use std::fs::File;
use std::io::Write;

pub fn parse_input() -> (GraphData, ExperimentType, File) {
    let args = std::env::args().collect::<Vec<_>>();

    let mut count = 1;

    let mut graphdata = None;
    let mut graph_file = None;

    let mut experimenttype = None;
    let mut input_file = None;

    let mut output_file = None;

    while count < args.len() {
        match &*args[count] {
            "-d" =>{
                count += 1;
                graph_file = Some(args[count].clone());
                graphdata = Some(Graph::from_file(&args[count]));
                count += 1;
            },
            "-ss" => {
                count += 1;
                input_file = Some(args[count].clone());
                let sources = parse_sources(&args[count]);
                count += 1;
                experimenttype = Some(ExperimentType::Ss(sources));
            },
            "-p2p" => {
                count += 1;
                input_file = Some(args[count].clone());
                let pairs = parse_pairs(&args[count]);
                count += 1;
                experimenttype = Some(ExperimentType::P2p(pairs));
            },
            "-oss" | "-op2p" => {
                count += 1;
                output_file = Some(args[count].clone());
                count += 1;
            },
            &_ => {
                count += 1;
                panic!("Invalid argument");
            }
        };
    }

    let mut msg = String::new();
    if graphdata.is_none() {
        msg += "Graph data not provided, ";
    }
    if experimenttype.is_none() {
        msg += "Experiment type not provided, ";
    }
    if output_file.is_none() {
        msg += "Output file not provided";
    }
    if !msg.is_empty() {
        println!("{}", msg);
        panic!("Invalid input");
    }

    let mut output = File::create(output_file.unwrap()).unwrap();
    write!(output,"c Twórca Programu: Paweł Stanik\n").unwrap();

    write!(output,"f {} {}\n", graph_file.unwrap(), input_file.unwrap()).unwrap();

    let graphdata = graphdata.unwrap();
    write!(output,"g {} {} {} {}\n", graphdata.vertices, graphdata.edges, graphdata.min_weight, graphdata.max_weight ).unwrap();

    (graphdata, experimenttype.unwrap(), output)
}

#[derive(Debug)]
pub enum ExperimentType{
    Ss(Vec<usize>),
    P2p(Vec<(usize,usize)>),
}