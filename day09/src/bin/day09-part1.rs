use clap::Parser;
use clio::Input;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Tile {
    x: usize,
    y: usize,
}
type Tiles = HashSet<Tile>;
type Pair<'a> = (&'a Tile, &'a Tile);

fn area(pair: Pair) -> usize {
    let dx = pair.0.x.abs_diff(pair.1.x) + 1;
    let dy = pair.0.y.abs_diff(pair.1.y) + 1;
    dx * dy
}

fn solve(lines: Vec<String>) -> usize {
    let mut floor = Tiles::new();
    for line in lines.iter() {
        let (x, y) = line
            .split_once(',')
            .map(|s| (s.0.parse::<usize>().unwrap(), s.1.parse::<usize>().unwrap()))
            .unwrap();
        floor.insert(Tile { x, y });
    }

    // Map the distances between the points
    let mut distances: Vec<usize> = floor
        .iter()
        .tuple_combinations()
        .par_bridge()
        .map(|pair| area(pair))
        .collect();

    // sort with the furthest at the end
    distances.sort();
    distances.pop().unwrap()
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
        let example = ["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 50);
    }
}
