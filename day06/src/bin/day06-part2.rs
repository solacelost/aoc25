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

    let mut numbers: Vec<Vec<char>> = Vec::new();
    for line in lines.iter() {
        for (x, chr) in line.chars().enumerate() {
            if x >= numbers.len() {
                numbers.push(Vec::new());
            }
            numbers[x].push(chr);
        }
    }
    let mut problems: Vec<Vec<usize>> = Vec::new();
    problems.push(Vec::new());
    let mut i = 0;
    for digits in numbers.iter() {
        let string = digits.into_iter().collect::<String>();
        let number = string.trim();
        if number == "" {
            i += 1;
            if i <= len {
                problems.push(Vec::new());
            }
            continue;
        }
        problems[i].push(number.parse::<usize>().unwrap());
    }

    let mut ret: usize = 0;
    for (i, problem) in problems.iter_mut().enumerate() {
        let mut answer: usize = problem.pop().unwrap();
        for new in problem.iter() {
            match symbols[i] {
                "*" => answer = answer * new,
                "+" => answer = answer + new,
                _ => panic!("We should only have plus and minus"),
            }
        }
        ret += answer;
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
            "123 328  51 64",
            " 45 64  387 23",
            "  6 98  215 314",
            "*   +   *   +",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 3263827);
    }
}
