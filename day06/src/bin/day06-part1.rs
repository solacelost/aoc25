use clap::Parser;
use clio::Input;
use std::io::{self, BufReader, prelude::*};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

fn solve(mut lines: Vec<String>) -> usize {
    let last_line = lines.pop().unwrap();
    let symbols: Vec<&str> = last_line.split_whitespace().collect();
    let len = symbols.len();
    let mut answers: Vec<usize> = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    for line in lines.iter() {
        let new: Vec<usize> = line
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        for i in 0..len {
            match symbols[i] {
                "*" => answers[i] = answers[i] * new[i],
                "+" => answers[i] = answers[i] + new[i],
                _ => panic!("We should only have plus and minus"),
            }
        }
    }
    answers.iter().sum()
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
            "123 328  51 64",
            " 45 64  387 23",
            "  6 98  215 314",
            "*   +   *   +",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 4277556);
    }
}
