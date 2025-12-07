use clap::Parser;
use clio::Input;
use std::fmt;
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
    SplitterHit,
    Beam,
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = match self {
            Position::Empty => write!(f, "."),
            Position::Start => write!(f, "S"),
            Position::Splitter => write!(f, "^"),
            Position::SplitterHit => write!(f, "^"),
            Position::Beam => write!(f, "|"),
        };
        Ok(())
    }
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
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for position in self.columns.iter() {
            let _ = write!(f, "{}", position);
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Manifold {
    rows: Vec<Row>,
    start: Option<(usize, usize)>,
}
impl Manifold {
    fn new() -> Self {
        Self {
            rows: Vec::new(),
            start: None,
        }
    }
    fn add(&mut self, row: Row) {
        self.rows.push(row);
    }
    fn process(&mut self, row_y: usize, beam_xs: Vec<usize>) -> Vec<usize> {
        let mut ret = Vec::new();
        let row = &mut self.rows[row_y];
        for beam_x in beam_xs {
            match row.columns[beam_x] {
                Position::Empty => {
                    row.columns[beam_x] = Position::Beam;
                    ret.push(beam_x);
                }
                Position::Splitter => {
                    row.columns[beam_x] = Position::SplitterHit;
                    ret.push(beam_x - 1);
                    ret.push(beam_x + 1);
                }
                _ => panic!("This should never happen"),
            }
        }
        ret.dedup();
        ret
    }
    fn beam(&mut self) {
        let max_row = self.rows.len();
        let (startx, starty) = self.start.unwrap();
        let mut beams = Vec::from([startx]);
        for row in (starty + 1)..max_row {
            beams = self.process(row, beams);
        }
    }
}
impl fmt::Display for Manifold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows.iter() {
            let _ = writeln!(f, "{}", row);
        }
        Ok(())
    }
}

fn solve(lines: Vec<String>) -> usize {
    let mut manifold = Manifold::new();
    let mut start: Option<(usize, usize)> = None;
    for (y, line) in lines.iter().enumerate() {
        let mut row = Row::new();
        for (x, chr) in line.chars().enumerate() {
            match chr {
                '.' => row.add(Position::Empty),
                'S' => {
                    row.add(Position::Start);
                    start = Some((x, y));
                }
                '^' => row.add(Position::Splitter),
                _ => panic!("This should never happen"),
            }
        }
        if let Some(pair) = start {
            manifold.start = Some(pair);
            start = None;
        }
        manifold.add(row);
    }
    manifold.beam();
    eprintln!("Processed:");
    eprintln!("{}", manifold);

    manifold
        .rows
        .into_iter()
        .map(|r| r.columns)
        .flatten()
        .filter(|p| matches!(p, Position::SplitterHit))
        .count()
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
        assert_eq!(solve(convert_example(&example)), 21);
    }
}
