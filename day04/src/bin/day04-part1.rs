use clap::Parser;
use clio::Input;
use rayon::prelude::*;
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

#[derive(Clone, Copy, Debug)]
struct Paper {
    x: usize,
    y: usize,
}
impl Paper {
    fn nearby(&self, grid: &Vec<Vec<Option<Paper>>>) -> usize {
        let mut ret = 0;
        let checks: [isize; 3] = [-1, 0, 1];
        for offset_y in checks.iter() {
            for offset_x in checks.iter() {
                if self.offset(*offset_x, *offset_y, grid).is_some() {
                    ret += 1;
                }
            }
        }
        ret
    }
    fn offset(
        &self,
        offset_x: isize,
        offset_y: isize,
        grid: &Vec<Vec<Option<Paper>>>,
    ) -> Option<Paper> {
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
type Grid = Vec<Vec<Option<Paper>>>;

fn solve(lines: Vec<String>) -> usize {
    let mut grid: Grid = Vec::new();

    for (y, row) in lines.iter().enumerate() {
        let mut v_row = Vec::new();
        for (x, char) in row.chars().enumerate() {
            if char == '@' {
                v_row.push(Some(Paper { x, y }));
            } else {
                v_row.push(None);
            }
        }
        grid.push(v_row);
    }
    grid.par_iter()
        .map(|row| {
            row.par_iter()
                .map(|p| match p {
                    Some(paper) => paper.nearby(&grid) < 4,
                    None => false,
                })
                .filter(|p| *p)
                .count()
        })
        .sum()
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
        assert_eq!(solve(convert_example(&example)), 13);
    }
}
