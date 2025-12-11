use clap::Parser;
use clio::Input;
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
        Regex::new(r"^\[([.#]*)\] (?<buttons>(\([0-9,]*\) )*)\{(?<joltage>[0-9,]*)\}").unwrap();
    lines
        .iter()
        .map(|line| {
            if let Some(capture) = re.captures(line) {
                let buttons: Vec<Vec<usize>> = capture["buttons"]
                    .split_terminator(" ")
                    .map(|b| {
                        b.trim_matches(['(', ')'])
                            .split(',')
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect()
                    })
                    .collect();

                let joltages: Vec<usize> = capture["joltage"]
                    .split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect();
                let max = joltages.iter().copied().max().unwrap() as i32;

                let mut problem = Problem::new(OptimizationDirection::Minimize);
                let mut vars: Vec<Variable> = Vec::new();
                for _ in 0..(buttons.len()) {
                    vars.push(problem.add_integer_var(1.0, (0, max)));
                }

                for (i, &joltage) in joltages.iter().enumerate() {
                    let mut button_constraint = LinearExpr::empty();
                    buttons
                        .iter()
                        .zip(&vars)
                        .filter(|(b, _)| b.contains(&i))
                        .for_each(|(_, &var)| {
                            button_constraint.add(var, 1.0);
                        });
                    problem.add_constraint(button_constraint, ComparisonOp::Eq, joltage as f64);
                }
                if let Ok(answer) = problem.solve() {
                    answer.objective().round() as usize
                } else {
                    panic!("Couldn't solve: {}", line);
                }
            } else {
                panic!("Couldn't parse: {}", line);
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
