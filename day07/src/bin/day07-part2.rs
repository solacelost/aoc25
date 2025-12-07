use clap::Parser;
use clio::Input;
use moka::sync::Cache;
use std::io::{self, BufReader, prelude::*};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

fn process(
    grid: &Vec<Vec<char>>,
    beam: (usize, usize),
    cache: &mut Cache<(usize, usize), usize>,
) -> usize {
    if beam.1 == grid.len() - 1 {
        return 1;
    }
    if let Some(cached) = cache.get(&beam) {
        return cached;
    }
    let row = &grid[beam.1];
    let ret = match row[beam.0] {
        '.' => process(grid, (beam.0, beam.1 + 1), cache),
        '^' => {
            process(grid, (beam.0 - 1, beam.1 + 1), cache)
                + process(grid, (beam.0 + 1, beam.1 + 1), cache)
        }
        _ => panic!(
            "Somehow hit position '{:?}' at ({}, {})",
            row[beam.0], beam.0, beam.1
        ),
    };
    cache.insert((beam.0, beam.1), ret);
    ret
}

fn solve(lines: Vec<String>) -> usize {
    let cache = Cache::new(10_000);

    let mut manifold = Vec::new();
    let mut start: Option<(usize, usize)> = None;
    for (y, line) in lines.iter().enumerate() {
        let mut row = Vec::new();
        for (x, chr) in line.chars().enumerate() {
            match chr {
                'S' => {
                    row.push(chr);
                    start = Some((x, y + 1));
                }
                _ => row.push(chr),
            }
        }
        manifold.push(row);
    }
    process(&manifold, start.unwrap(), &mut cache.clone())
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
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 40);
    }
}
