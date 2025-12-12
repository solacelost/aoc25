use clap::Parser;
use clio::Input;
use hashbrown::HashMap;
use std::io::{self, BufReader, prelude::*};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

fn paths<'a>(
    map: &HashMap<&str, Vec<&'a str>>,
    from: &'a str,
    to: &str,
    visited: &mut HashMap<&'a str, usize>,
) -> usize {
    if from == to {
        return 1;
    }
    if let Some(value) = visited.get(from) {
        return *value;
    }
    let mut ret = 0;
    if let Some(references) = map.get(from) {
        for reference in references.iter() {
            let count = paths(map, reference, to, visited);
            ret += count;
        }
    }
    visited.insert(from, ret);
    ret
}

fn solve(lines: Vec<String>) -> usize {
    let map: HashMap<&str, Vec<&str>> = lines
        .iter()
        .map(|line| {
            let (node, relationships) = line.split_once(":").unwrap();
            (node, relationships.split_whitespace().collect())
        })
        .collect();

    let mut ret = 1;
    for (from, to) in [("svr", "fft"), ("fft", "dac"), ("dac", "out")].iter() {
        let mut visited = HashMap::new();
        ret *= paths(&map, from, to, &mut visited);
    }
    ret
}

fn main() -> io::Result<()> {
    let opt = Opt::parse();

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
            "svr: aaa bbb",
            "aaa: fft",
            "fft: ccc",
            "bbb: tty",
            "tty: ccc",
            "ccc: ddd eee",
            "ddd: hub",
            "hub: fff",
            "eee: dac",
            "dac: fff",
            "fff: ggg hhh",
            "ggg: out",
            "hhh: out",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 2);
    }
}
