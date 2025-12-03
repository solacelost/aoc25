use clap::Parser;
use clio::Input;
use rayon::prelude::*;
use std::fmt;
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
        let mut ret = 0;
        let starting_point = self.starting_point(9);
        //eprintln!("Found starting point in {}: {:?}", self, starting_point);
        ret += starting_point.1 * usize::pow(10, 11);
        let mut next = starting_point.clone();
        for i in (0..11).rev() {
            next = self._next_digit(next, i + 1);
            //eprintln!("Next digit for {}: {:?}", self, next);
            let pow = usize::pow(10, i as u32);
            ret += pow * next.1;
        }
        //eprintln!("Joltage for {}: {}", self, ret);
        ret
    }

    fn starting_point(&self, max_digit: usize) -> (usize, usize) {
        let highest = self._highest(None, max_digit);
        if self._can_fit_12_after(highest) {
            return highest;
        }
        self.starting_point(max_digit - 1)
    }

    fn _next_digit(&self, battery: (usize, usize), remain: usize) -> (usize, usize) {
        let mut next = self._highest(Some(battery.0), 9);
        while !self._can_fit_n_after(next, remain) {
            //eprintln!("Cannot fit {} digits after candidate {:?}", remain, next);
            next = self._highest(Some(battery.0), next.1 - 1);
        }
        //eprintln!("Found {:?} to fit after {:?}", next, battery);
        next
    }

    fn _can_fit_n_after(&self, battery: (usize, usize), n: usize) -> bool {
        self.batteries.len() - battery.0 >= n
    }
    fn _can_fit_12_after(&self, battery: (usize, usize)) -> bool {
        self._can_fit_n_after(battery, 12)
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
impl fmt::Display for Bank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for battery in self.batteries.iter() {
            let _ = write!(f, "{}", battery);
        }
        Ok(())
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
        assert_eq!(solve(convert_example(&example)), 3121910778619);
    }
}
