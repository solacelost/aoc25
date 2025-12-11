use clap::Parser;
use clio::Input;
use itertools::Itertools;
use microlp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem, Variable};
use regex::Regex;
use std::io::{self, BufReader, prelude::*};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

fn solve(lines: Vec<String>) -> usize {
    let re =
        Regex::new(r"\[(?<indicators>[.#]*)\] (?<buttons>(\([0-9,]*\) )*)\{(?<joltage>[0-9,]*)\}")
            .unwrap();
    lines
        .iter()
        .map(|line| {
            if let Some(capture) = re.captures(line) {
                //let indicators: Vec<bool> = capture["indicators"].chars().map(|c| c == '#').collect();
                let buttons: Vec<Vec<usize>> = capture["buttons"]
                    .split_terminator(" ")
                    .map(|b| {
                        b.trim_matches(['(', ')'])
                            .split(',')
                            .map(|n| n.parse::<usize>().unwrap())
                            .sorted()
                            .collect()
                    })
                    .collect();
                //eprintln!("Parsed buttons: {:?}", buttons);

                let joltage: Vec<usize> = capture["joltage"]
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();
                //eprintln!("Parsed joltage: {:?}", joltage);

                let mut problem = Problem::new(OptimizationDirection::Minimize);
                let max = joltage.iter().copied().max().unwrap();
                //eprintln!("Max joltage: {}", max);

                let vars: Vec<Variable> = (0..(buttons.len()))
                    .map(|_| problem.add_integer_var(1.0, (0, max as i32)))
                    .collect();
                //eprintln!("Created {} variables: {:?}", buttons.len(), vars);

                for (i, &n) in joltage.iter().enumerate() {
                    let expr = buttons
                        .iter()
                        .zip(&vars)
                        //.map(|t| {
                        //    eprintln!("Zipped button/var: {:?}", t);
                        //    t
                        //})
                        .filter(|(b, _)| b.contains(&i))
                        //.map(|t| {
                        //    eprintln!("Filtered to {:?}", t);
                        //    t
                        //})
                        .fold(LinearExpr::empty(), |mut ex, (_, &var)| {
                            ex.add(var, 1.0);
                            ex
                        });
                    //eprintln!("Created linear expression for position {}: {:?}", i, expr);
                    problem.add_constraint(expr, ComparisonOp::Eq, n as f64);
                }
                if let Ok(answer) = problem.solve() {
                    //eprintln!("Found answer: {:?}", answer);
                    answer.objective().round() as usize
                } else {
                    //eprintln!("No answer!");
                    0
                }
            } else {
                //eprintln!("Failed parsing!");
                0
            }
        })
        .sum()
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
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
            "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
            "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 33);
    }
}
