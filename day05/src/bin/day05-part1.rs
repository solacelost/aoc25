use clap::Parser;
use clio::Input;
use std::io::{self, BufReader, prelude::*};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

fn is_fresh(id: usize, fresh: &Vec<(usize, usize)>) -> bool {
    for range in fresh.iter() {
        if id >= range.0 && id <= range.1 {
            return true;
        }
    }
    false
}

fn solve(lines: Vec<String>) -> usize {
    let mut fresh: Vec<(usize, usize)> = Vec::new();
    let mut ids: Vec<usize> = Vec::new();
    let mut working_on_ids = false;
    for line in lines.iter() {
        if !working_on_ids {
            if line == "" {
                working_on_ids = true;
                continue;
            }
            let (left, right) = line.split_once('-').unwrap();
            fresh.push((
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            ));
        } else {
            ids.push(line.parse::<usize>().unwrap());
        }
    }

    ids.iter()
        .map(|id| is_fresh(*id, &fresh))
        .filter(|is_fresh| *is_fresh)
        .count()
}

fn main() -> io::Result<()> {
    let opt = Opt::parse();

    let reader = BufReader::new(opt.input);
    let lines: Vec<String> = reader.lines().flatten().collect();
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
            "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 3);
    }
}
