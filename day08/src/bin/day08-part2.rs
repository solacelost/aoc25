use clap::Parser;
use clio::Input;
use itertools::Itertools;
use petgraph::algo::connected_components;
use petgraph::graph::{NodeIndex, UnGraph};
use rayon::prelude::*;
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

type JunctionBox = (usize, usize, usize);
type Playground = Vec<JunctionBox>;

fn euclidean_distance(a: &JunctionBox, b: &JunctionBox) -> usize {
    let dx = a.0.abs_diff(b.0);
    let dy = a.1.abs_diff(b.1);
    let dz = a.2.abs_diff(b.2);
    (dx.pow(2) + dy.pow(2) + dz.pow(2)).isqrt()
}

fn solve(lines: Vec<String>) -> usize {
    // Parse the junction boxes
    let playground: Playground = lines
        .iter()
        .map(|line| {
            line.splitn(3, ",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    // prepare the graph of indexes
    let mut circuit = UnGraph::<usize, ()>::new_undirected();
    let nodes: Vec<NodeIndex> = (0..playground.len()).map(|i| circuit.add_node(i)).collect();

    // record the distance between all pairs, store the indexes in the playground
    let mut sorted_distances: Vec<((usize, usize), usize)> = playground
        .iter()
        .enumerate()
        .tuple_combinations()
        .par_bridge()
        .map(|((i, a), (j, b))| {
            let distance = euclidean_distance(a, b);
            ((i, j), distance)
        })
        .collect();
    sorted_distances.par_sort_by(|a, b| b.1.cmp(&a.1));

    // graph all the nodes in the playground
    loop {
        let ((l, r), _) = sorted_distances.pop().unwrap();
        circuit.update_edge(nodes[l], nodes[r], ());
        if connected_components(&circuit) == 1 {
            return playground[l].0 * playground[r].0;
        }
    }
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
            "162,817,812",
            "57,618,57",
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",
            "431,825,988",
            "739,650,466",
            "52,470,668",
            "216,146,977",
            "819,987,18",
            "117,168,530",
            "805,96,715",
            "346,949,466",
            "970,615,88",
            "941,993,340",
            "862,61,35",
            "984,92,344",
            "425,690,689",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 25272);
    }
}
