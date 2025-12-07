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

#[derive(Debug, Copy, Clone)]
enum Position {
    Empty,
    Start,
    Splitter,
}

#[derive(Debug, Clone)]
struct Row {
    columns: Vec<Position>,
}
impl Row {
    fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }
    fn add(&mut self, position: Position) {
        self.columns.push(position);
    }
}

#[derive(Debug)]
struct Manifold {
    rows: Vec<Row>,
}
impl Manifold {
    fn new() -> Self {
        Self { rows: Vec::new() }
    }
    fn add(&mut self, row: Row) {
        self.rows.push(row);
    }
    fn process(&self, beam: (usize, usize), cache: &mut Cache<(usize, usize), usize>) -> usize {
        if beam.1 == self.rows.len() - 1 {
            return 1;
        }
        if let Some(result) = cache.get(&beam) {
            return result;
        }
        let row = &self.rows[beam.1];
        let ret = match row.columns[beam.0] {
            Position::Empty => self.process((beam.0, beam.1 + 1), cache),
            Position::Splitter => {
                self.process((beam.0 - 1, beam.1 + 1), cache)
                    + self.process((beam.0 + 1, beam.1 + 1), cache)
            }
            _ => panic!(
                "Somehow hit position '{:?}' at ({}, {})",
                row.columns[beam.0], beam.0, beam.1
            ),
        };
        cache.insert((beam.0, beam.1), ret);
        ret
    }
}

fn solve(lines: Vec<String>) -> usize {
    let cache = Cache::new(1_000_000);

    let mut manifold = Manifold::new();
    let mut start: Option<(usize, usize)> = None;
    for (y, line) in lines.iter().enumerate() {
        let mut row = Row::new();
        for (x, chr) in line.chars().enumerate() {
            match chr {
                '.' => row.add(Position::Empty),
                'S' => {
                    row.add(Position::Start);
                    start = Some((x, y + 1));
                }
                '^' => row.add(Position::Splitter),
                _ => panic!("This should never happen"),
            }
        }
        manifold.add(row);
    }
    manifold.process(start.unwrap(), &mut cache.clone())
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
