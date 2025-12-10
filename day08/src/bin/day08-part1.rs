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

type JunctionBox = (usize, usize, usize);
type Pair = (JunctionBox, JunctionBox);
type Playground = Vec<JunctionBox>;
type Circuit = HashSet<JunctionBox>;

fn euclidean_distance(pair: Pair) -> usize {
    let dx = pair.0.0.abs_diff(pair.1.0);
    let dy = pair.0.1.abs_diff(pair.1.1);
    let dz = pair.0.2.abs_diff(pair.1.2);
    (dx.pow(2) + dy.pow(2) + dz.pow(2)).isqrt()
}

fn pair_in_circuit(pair: Pair, circuit: &Circuit) -> bool {
    let (l, r) = pair;
    if circuit.get(&l).is_some() {
        return true;
    }
    if circuit.get(&r).is_some() {
        return true;
    }
    false
}

fn solve(lines: Vec<String>, count: usize) -> usize {
    // Parse the junction boxes
    let mut playground: Playground = Vec::new();
    for line in lines.iter() {
        let junction: JunctionBox = line
            .splitn(3, ",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        playground.push(junction);
    }
    // record the distance between all pairs, store them
    let mut distances: Vec<(Pair, usize)> = playground
        .into_iter()
        .tuple_combinations()
        .par_bridge()
        .map(|pair| {
            let distance = euclidean_distance(pair);
            (pair, distance)
        })
        .collect();
    // sort with the closest at the end
    distances.sort_by(|a, b| b.1.cmp(&a.1));

    // map $count worth of close pairs from the whole playground
    let mut circuits: Vec<Circuit> = Vec::new();
    for _ in 0..count {
        // this is the closest pair
        let (pair, _) = distances.pop().unwrap();

        // check if we already have a circuit with one of these junction boxes in it
        let mut to_add: Option<usize> = None;
        for (i, circuit) in circuits.iter().enumerate() {
            if pair_in_circuit(pair, circuit) {
                to_add = Some(i);
                break;
            }
        }
        // if we do, add this pair to the circuit, otherwise create a new circuit with the pair
        if let Some(i) = to_add {
            circuits[i].insert(pair.0);
            circuits[i].insert(pair.1);
        } else {
            circuits.push(HashSet::from([pair.0, pair.1]));
        }
    }

    // merge all connected circuits
    loop {
        let mut mergable: Option<(usize, usize)> = None;
        'outer: for i in 0..circuits.len() {
            for j in 0..circuits.len() {
                if i == j {
                    continue;
                }
                let intersection: Vec<&JunctionBox> =
                    circuits[i].intersection(&circuits[j]).collect();
                if intersection.len() != 0 {
                    mergable = Some((i, j));
                    break 'outer;
                }
            }
        }
        if mergable.is_none() {
            break;
        }
        let (l, r) = mergable.unwrap();
        let mut left: Circuit;
        let right: Circuit;
        if l < r {
            right = circuits.remove(r);
            left = circuits.remove(l);
        } else {
            left = circuits.remove(l);
            right = circuits.remove(r);
        }
        left.extend(right);
        circuits.push(left);
    }

    // sort the circuits with longest first
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    // multiply the length of the three longest circuits
    let mut ret = 1;
    for circuit in circuits.iter().take(3) {
        ret *= circuit.len();
    }
    ret
}

fn main() -> io::Result<()> {
    let opt = Opt::parse();

    rayon::ThreadPoolBuilder::new()
        .num_threads(opt.threads)
        .build_global()
        .unwrap();

    let reader = BufReader::new(opt.input);
    let lines: Vec<String> = reader.lines().flatten().filter(|s| !s.is_empty()).collect();
    println!("{}", solve(lines, 1000));
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
        assert_eq!(solve(convert_example(&example), 10), 40);
    }
}
