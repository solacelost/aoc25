use clap::Parser;
use clio::Input;
use std::io::{self, BufReader, prelude::*};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

#[derive(Clone, Copy, Debug)]
struct Paper {
    x: usize,
    y: usize,
}
impl Paper {
    fn nearby(&self, grid: &Grid) -> Vec<Paper> {
        let checks: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        checks
            .iter()
            .map(|c| self.offset(c.0, c.1, grid))
            .flatten()
            .collect()
    }
    fn offset(&self, offset_x: isize, offset_y: isize, grid: &Grid) -> Option<Paper> {
        let max_y = grid.len() as isize;
        let max_x = grid[0].len() as isize;
        if offset_x < 0 {
            if self.x == 0 {
                return None;
            }
        }
        if offset_x + self.x as isize >= max_x {
            return None;
        }
        if offset_y < 0 {
            if self.y == 0 {
                return None;
            }
        }
        if offset_y + self.y as isize >= max_y {
            return None;
        }
        if offset_y == 0 && offset_x == 0 {
            return None;
        }
        let x = (self.x as isize + offset_x) as usize;
        let y = (self.y as isize + offset_y) as usize;
        grid[y][x]
    }
}
type Row = Vec<Option<Paper>>;
type Grid = Vec<Row>;

fn removable(grid: &Grid) -> Vec<&Paper> {
    grid.iter()
        .map(|row| {
            row.iter()
                .filter(|option| {
                    if let Some(paper) = option {
                        paper.nearby(grid).len() < 4
                    } else {
                        false
                    }
                })
                .flatten()
        })
        .flatten()
        .collect()
}

fn solve(lines: Vec<String>) -> usize {
    let mut grid = Grid::new();

    for (y, row) in lines.iter().enumerate() {
        let mut v_row = Row::new();
        for (x, char) in row.chars().enumerate() {
            if char == '@' {
                v_row.push(Some(Paper { x, y }));
            } else {
                v_row.push(None);
            }
        }
        grid.push(v_row);
    }
    let mut done = false;
    let mut ret = 0;
    while !done {
        let start = &grid.clone();
        let to_remove = removable(start);
        if to_remove.len() == 0 {
            done = true;
        }
        for paper in to_remove.iter() {
            grid[paper.y][paper.x] = None;
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
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 43);
    }
}
