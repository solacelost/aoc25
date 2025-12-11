use clap::Parser;
use clio::Input;
use petgraph::algo::simple_paths::all_simple_paths;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::hash_map::RandomState;
use std::io::{self, BufReader, prelude::*};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,

    /// the number of CPU cores to use (all if unspecified)
    #[clap(short, long, default_value_t = num_cpus::get())]
    threads: usize,
}

fn solve(lines: Vec<String>) -> usize {
    let mut wiring = DiGraph::<usize, ()>::new();
    let mut map: Vec<(&str, Vec<&str>)> = lines
        .iter()
        .map(|line| {
            let (node, relationships) = line.split_once(":").unwrap();
            (node, relationships.split_whitespace().collect())
        })
        .collect();
    map.push(("out", Vec::new()));
    let nodes: Vec<NodeIndex> = (0..map.len()).map(|i| wiring.add_node(i)).collect();
    map.iter()
        .enumerate()
        .for_each(|(src, (_, relationships))| {
            for other in relationships.iter() {
                let dest = map
                    .iter()
                    .position(|&(othernode, _)| &othernode == other)
                    .unwrap();
                wiring.add_edge(nodes[src], nodes[dest], ());
            }
        });
    let you = nodes[map
        .iter()
        .position(|&(othernode, _)| othernode == "you")
        .unwrap()];
    let out = nodes[map.len() - 1];
    all_simple_paths::<Vec<_>, _, RandomState>(&wiring, you, out, 0, None).count()
}

fn main() -> io::Result<()> {
    let opt = Opt::parse();

    rayon::ThreadPoolBuilder::new()
        .num_threads(opt.threads)
        .build_global()
        .unwrap();

    let reader = BufReader::new(opt.input);
    let lines: Vec<String> = reader.lines().flatten().filter(|s| !s.is_empty()).collect();
    println!("{}", solve(lines));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn convert_example(example: &[&str]) -> Vec<String> {
        example.iter().map(|line| line.to_string()).collect()
    }

    #[test]
    fn given() {
        let example = [
            "aaa: you hhh",
            "you: bbb ccc",
            "bbb: ddd eee",
            "ccc: ddd eee fff",
            "ddd: ggg",
            "eee: out",
            "fff: out",
            "ggg: out",
            "hhh: ccc fff iii",
            "iii: out",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 5);
    }
}
