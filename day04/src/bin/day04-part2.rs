use clap::Parser;
use clio::Input;
use std::fmt;
use std::io::{self, BufReader, prelude::*};
use std::ops::{Index, IndexMut};

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
        let mut ret = Vec::new();
        let checks: [isize; 3] = [-1, 0, 1];
        for offset_y in checks.iter() {
            for offset_x in checks.iter() {
                if let Some(paper) = self.offset(*offset_x, *offset_y, grid) {
                    //eprintln!("Found {:?} adjacent to {:?}", paper, self);
                    ret.push(paper);
                }
            }
        }
        //eprintln!("Paper {:?} has {} nearby nodes", self, ret);
        ret
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
#[derive(Clone, Debug)]
struct Row {
    columns: Vec<Option<Paper>>,
}
impl Row {
    fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }
    fn push(&mut self, other: Option<Paper>) {
        self.columns.push(other);
    }
    fn len(&self) -> usize {
        self.columns.len()
    }
}
impl Index<usize> for Row {
    type Output = Option<Paper>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.columns[index]
    }
}
impl IndexMut<usize> for Row {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.columns[index]
    }
}
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for paper in self.columns.iter() {
            if paper.is_some() {
                let _ = write!(f, "@");
            } else {
                let _ = write!(f, ".");
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug)]
struct Grid {
    rows: Vec<Row>,
}
impl Grid {
    fn new() -> Self {
        Self { rows: Vec::new() }
    }
    fn push(&mut self, other: Row) {
        self.rows.push(other);
    }
    fn removable(&self) -> Vec<&Paper> {
        let mut ret = Vec::new();
        for row in self.rows.iter() {
            for option in row.columns.iter() {
                if let Some(paper) = option {
                    if paper.nearby(&self).len() < 4 {
                        ret.push(paper);
                    }
                }
            }
        }
        ret
    }
    fn len(&self) -> usize {
        self.rows.len()
    }
}
impl Index<usize> for Grid {
    type Output = Row;
    fn index(&self, index: usize) -> &Self::Output {
        &self.rows[index]
    }
}
impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.rows[index]
    }
}
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.rows.iter() {
            let _ = writeln!(f, "{}", row);
        }
        Ok(())
    }
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
        let start = grid.clone();
        //eprintln!("Checking for removable paper in:");
        //eprintln!("{}", start);
        let removable = start.removable();
        //eprintln!("Found {}: {:?}", removable.len(), removable);

        if removable.len() == 0 {
            done = true;
        }
        for paper in removable.into_iter() {
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
