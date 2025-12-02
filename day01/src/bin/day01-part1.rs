use clap::Parser;
use clio::Input;
use std::io::{self, BufReader, prelude::*};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

fn solve(lines: Vec<String>) -> usize {
    let mut dial: isize = 50;
    let mut ret = 0;
    for line in lines.iter() {
        let direction = line.chars().nth(0).unwrap();
        let mut num = line[1..line.len()].parse::<isize>().unwrap();
        if direction == 'L' {
            num = -num;
        }
        dial = (dial + num) % 100;
        if dial == 0 {
            ret += 1;
        }
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
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 3);
    }
}
