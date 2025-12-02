use clap::Parser;
use clio::Input;
use std::io::{self, BufReader, prelude::*};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

fn solve(lines: Vec<String>) -> isize {
    let mut dial = 50;
    //eprintln!("Starting at {}", dial);
    let mut ret = 0;
    for line in lines.iter() {
        let direction = line.chars().nth(0).unwrap();
        let mut num = line[1..line.len()].parse::<isize>().unwrap();
        if direction == 'L' {
            //eprintln!("Moving Left by {}", num);
            num = -num;
            if dial + num <= 0 {
                let mut start = 0;
                if dial != 0 {
                    start = 1;
                }
                let sweeps = start + ((dial + num) / 100).abs();
                if sweeps != 0 {
                    //eprintln!("Swept by, or landed on, 0 {} times", sweeps);
                }
                ret += sweeps;
            }
        } else {
            //eprintln!("Moving Right by {}", num);
            let sweeps = (dial + num) / 100;
            if sweeps != 0 {
                //eprintln!("Swept by, or landed on, 0 {} times", sweeps);
            }
            ret += sweeps;
        }
        dial = (dial + num).rem_euclid(100);
        //eprintln!("Landed on {}", dial);
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
        assert_eq!(solve(convert_example(&example)), 6);
    }
    #[test]
    fn extra() {
        let example = [
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82", "R1000", "L1000",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 26);
    }
    #[test]
    fn input_one() {
        let example = ["L23", "R14", "L50"];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 1);
    }
    #[test]
    fn input_two() {
        let example = ["L23", "R14", "L50", "R27"];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 2);
    }
    #[test]
    fn right_sweeps() {
        let example = ["L23", "R14", "L50", "R27", "L18", "R437"];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 7);
    }
}
