use clap::Parser;
use clio::Input;
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::io::{self, BufReader, prelude::*};
use std::ops::{Range, RangeInclusive};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,

    /// the number of CPU cores to use (all if unspecified)
    #[clap(short, long, default_value_t = num_cpus::get())]
    threads: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Tile {
    Red,
    Green,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}
type Floor = Vec<Vec<Option<Tile>>>;
type Pair<'a> = (&'a Coord, &'a Coord);
type Coords = Vec<Coord>;

fn area(pair: Pair, floor: &Floor) -> usize {
    let left: &Coord;
    let right: &Coord;
    // order the coords left to right
    if pair.0.x < pair.1.x {
        left = pair.0;
        right = pair.1;
    } else {
        left = pair.1;
        right = pair.0;
    }
    // determine which vertical direction we should range
    let vertical: RangeInclusive<usize>;
    if left.y < right.y {
        vertical = left.y..=right.y;
    } else {
        vertical = right.y..=left.y;
    }
    // check every coord in the two rays forming the sides of the rectangle beween these two
    for y in vertical {
        if floor[y][left.x].is_none() {
            return 0;
        }
    }
    for x in left.x..=right.x {
        if floor[left.y][x].is_none() {
            return 0;
        }
    }

    // take the area
    let dist_x = pair.0.x.abs_diff(pair.1.x) + 1;
    let dist_y = pair.0.y.abs_diff(pair.1.y) + 1;
    dist_x * dist_y
}

fn lay_green(a: Coord, b: Coord, floor: &mut Floor) {
    if a.x == b.x {
        let ty_range: Range<usize>;
        if a.y < b.y {
            ty_range = (a.y + 1)..b.y;
        } else {
            ty_range = (b.y + 1)..a.y;
        }
        for ty in ty_range {
            floor[ty][b.x] = Some(Tile::Green);
        }
    } else {
        let tx_range: Range<usize>;
        if a.x < b.x {
            tx_range = (a.x + 1)..b.x;
        } else {
            tx_range = (b.x + 1)..a.x;
        }
        for tx in tx_range {
            floor[b.y][tx] = Some(Tile::Green);
        }
    }
}

fn solve(lines: Vec<String>) -> usize {
    let mut coords = Coords::new();
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    //eprintln!("Parsing");
    for line in lines.iter() {
        let (x, y) = line
            .split_once(',')
            .map(|s| (s.0.parse::<usize>().unwrap(), s.1.parse::<usize>().unwrap()))
            .unwrap();
        let coord = Coord { x, y };
        coords.push(coord);
        if y > max_y {
            max_y = y;
        }
        if x > max_x {
            max_x = x;
        }
    }

    let mut floor = Floor::new();
    let mut row = Vec::new();
    row.resize(max_x + 1, None);
    floor.resize(max_y + 1, row.clone());

    let mut first: Option<Coord> = None;
    let mut last: Option<Coord> = None;
    let mut prev: Option<Coord> = None;

    //eprintln!("Laying tile borders");

    for coord in coords.iter() {
        if first.is_none() {
            first = Some(*coord);
        }
        floor[coord.y][coord.x] = Some(Tile::Red);
        if let Some(prev_c) = prev {
            lay_green(*coord, prev_c, &mut floor);
        }
        last = Some(*coord);
        prev = Some(*coord);
    }

    let last = last.unwrap();
    let first = first.unwrap();
    lay_green(last, first, &mut floor);

    //eprintln!("Finding all tiles");
    let mut filled_tiles: Vec<Coord> = floor
        .par_iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, t)| t.is_some())
                .map(move |(x, _)| Coord { x, y })
                .collect()
        })
        .reduce(
            || Vec::new(),
            |mut a: Vec<Coord>, mut b: Vec<Coord>| {
                a.append(&mut b);
                a
            },
        );
    filled_tiles.par_sort_by(|a, b| {
        if a.y > b.y {
            Ordering::Greater
        } else if b.y > a.y {
            Ordering::Less
        } else if a.x > b.x {
            Ordering::Greater
        } else if b.x > a.x {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });

    //eprintln!("Laying tile fill");
    let mut last: Option<Coord> = None;
    for tile in filled_tiles.iter() {
        if let Some(l) = last {
            if tile.y == l.y {
                if tile.x - l.x > 1 {
                    lay_green(l, *tile, &mut floor);
                }
            }
        }
        last = Some(*tile);
    }

    //eprintln!("Running area calculations");
    coords
        .iter()
        .tuple_combinations()
        .par_bridge()
        .map(|pair| area(pair, &floor))
        .max()
        .unwrap()
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
        let example = ["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 24);
    }
}
