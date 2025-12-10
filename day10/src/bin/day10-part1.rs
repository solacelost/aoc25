use clap::Parser;
use clio::Input;
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
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

#[derive(Debug)]
struct Button {
    toggles: Vec<usize>,
}
#[derive(Debug)]
struct Machine {
    indicators: Vec<bool>,
    buttons: Vec<Button>,
    //joltage: Vec<usize>,
}
type Machines = Vec<Machine>;

impl Machine {
    fn fewest(&self) -> usize {
        //eprintln!("Solving: {:?}", self);
        for button_combo in self.buttons.iter().powerset() {
            let mut test = self.indicators.clone();
            for button in button_combo.iter() {
                for i in button.toggles.iter() {
                    test[*i] = !test[*i];
                }
            }
            //eprintln!("Combo: {:?}", button_combo);
            //eprintln!("Test result: {:?}", test);
            if test.into_iter().filter(|b| *b).count() == 0 {
                //eprintln!("Solved with {} buttons!", button_combo.len());
                return button_combo.len();
            }
        }
        panic!("Could not find a combination!");
    }
}

fn solve(lines: Vec<String>) -> usize {
    let mut machines = Machines::new();
    let re =
        Regex::new(r"\[(?<indicators>[.#]*)\] (?<buttons>(\([0-9,]*\) )*)\{(?<joltage>[0-9,]*)\}")
            .unwrap();
    for line in lines.iter() {
        if let Some(capture) = re.captures(line) {
            let indicators: Vec<bool> = capture["indicators"].chars().map(|c| c == '#').collect();
            let buttons: Vec<Button> = capture["buttons"]
                .split_terminator(" ")
                .map(|b| Button {
                    toggles: b
                        .trim_matches(['(', ')'])
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect(),
                })
                .collect();
            //let joltage: Vec<usize> = capture["joltage"]
            //    .split(',')
            //    .map(|n| n.parse::<usize>().unwrap())
            //    .collect();
            machines.push(Machine {
                indicators,
                buttons,
                //    joltage,
            });
        }
    }
    machines.par_iter().map(|m| m.fewest()).sum()
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
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 7);
    }
}
