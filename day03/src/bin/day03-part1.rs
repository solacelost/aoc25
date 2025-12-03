use clap::Parser;
use clio::Input;
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

struct Bank {
    batteries: Vec<usize>,
}

impl Bank {
    fn new() -> Self {
        Self {
            batteries: Vec::new(),
        }
    }
    fn add(&mut self, battery: usize) {
        self.batteries.push(battery);
    }

    fn highest(&self) -> usize {
        let highest = self._highest(None, 9);
        let next_highest = self._highest(Some(highest.0), 9);
        if next_highest.1 == 0 {
            let second_highest = self._highest(None, highest.1 - 1);
            let pair = second_highest.1 * 10 + highest.1;
            eprintln!("Found (second candidate) highest pair: {}", pair);
            return pair;
        }
        let pair = highest.1 * 10 + next_highest.1;
        eprintln!("Found highest pair: {}", pair);
        pair
    }
    fn _highest(&self, other: Option<usize>, max_digit: usize) -> (usize, usize) {
        let starting_point = other.unwrap_or_else(|| 0);
        let mut ret: (usize, usize) = (0, 0);
        for (i, battery) in self.batteries.iter().enumerate() {
            if i <= starting_point && other.is_some() {
                continue;
            }
            if *battery == max_digit {
                return (i, *battery);
            }
            if *battery > ret.1 && *battery <= max_digit {
                ret = (i, *battery);
            }
        }
        ret
    }
}

struct PowerSupply {
    banks: Vec<Bank>,
}

impl PowerSupply {
    fn new() -> Self {
        Self { banks: Vec::new() }
    }
    fn add(&mut self, bank: Bank) {
        self.banks.push(bank);
    }
}

fn solve(lines: Vec<String>) -> usize {
    let mut supply = PowerSupply::new();
    for line in lines {
        let mut bank = Bank::new();
        for char in line.chars() {
            let maybe_digit = char.to_digit(10);
            if let Some(digit) = maybe_digit {
                bank.add(digit as usize)
            }
        }
        supply.add(bank);
    }
    supply.banks.par_iter().map(|b| b.highest()).sum()
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
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 357);
    }
}
