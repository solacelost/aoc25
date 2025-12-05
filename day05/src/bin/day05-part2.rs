use clap::Parser;
use clio::Input;
use std::io::{self, BufReader, prelude::*};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

fn is_in(id: usize, range: &(usize, usize)) -> bool {
    id >= range.0 && id <= range.1
}

fn merge(
    indexes: Vec<usize>,
    to_add: (usize, usize),
    ranges: &mut Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    //eprintln!(
    //    "Adding {:?} to {:?}, merging with indexes {:?}",
    //    to_add, ranges, indexes
    //);
    let mut new: (usize, usize) = to_add.clone();
    let mut updated = Vec::new();
    for i in indexes.iter().rev() {
        let old = ranges.remove(*i);
        if is_in(new.0, &old) {
            //eprintln!("Merging {:?} with {:?} on the right", new, old);
            new = (old.0, new.1);
            //eprintln!("Got: {:?}", new);
        }
        if is_in(new.1, &old) {
            //eprintln!("Merging {:?} with {:?} on the left", new, old);
            new = (new.0, old.1);
            //eprintln!("Got: {:?}", new);
        }
    }
    //eprintln!("Pushing: {:?}", new);
    updated.push(new);
    for range in ranges.into_iter() {
        updated.push(*range);
    }
    //eprintln!("Got: {:?}", updated);
    updated
}

fn solve(lines: Vec<String>) -> usize {
    let mut fresh: Vec<(usize, usize)> = Vec::new();
    for line in lines.iter() {
        if line == "" {
            break;
        }
        let (left, right) = line.split_once('-').unwrap();
        let l = left.parse::<usize>().unwrap();
        let r = right.parse::<usize>().unwrap();
        let new = (l, r);
        //eprintln!("Processing pair: {:?}", new);
        let mut to_merge: Vec<usize> = Vec::new();
        for (i, range) in fresh.iter().enumerate() {
            //eprintln!("Checking if {:?} is inside {:?}", new, range);
            if is_in(r, range) {
                to_merge.push(i);
            }
            if is_in(l, range) {
                to_merge.push(i);
            }
            //eprintln!("Checking if {:?} is inside {:?}", range, new);
            if is_in(range.1, &new) {
                to_merge.push(i);
            }
            if is_in(range.0, &new) {
                to_merge.push(i);
            }
        }
        if !to_merge.is_empty() {
            to_merge.dedup();
            //eprintln!("Found indexes needing merge: {:?}", to_merge);
            fresh = merge(to_merge, new, &mut fresh);
            continue;
        }
        fresh.push(new);
    }
    //for range in fresh.iter() {
    //    let mut warn: String = String::new();
    //    for inner in fresh.iter() {
    //        if range == inner {
    //            continue;
    //        }
    //        if is_in(range.0, inner) {
    //            warn = format!(" Found {} inside {:?}!", range.0, inner);
    //            break;
    //        }
    //        if is_in(range.1, inner) {
    //            warn = format!(" Found {} inside {:?}!", range.1, inner);
    //            break;
    //        }
    //    }
    //    eprintln!("{}-{}{}", range.0, range.1, warn);
    //}
    //eprintln!("Final: {:?}", fresh);
    fresh.iter().map(|(l, r)| (*l..=*r).count()).sum()
}

fn main() -> io::Result<()> {
    let opt = Opt::parse();

    let reader = BufReader::new(opt.input);
    let lines: Vec<String> = reader.lines().flatten().collect();
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
            "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 14);
    }
    #[test]
    fn problem_children() {
        let example = [
            "54611041342437-57224634168756",
            "51945266971870-59698236253713",
        ];
        println!("{}", example.join("\n"));
        assert_eq!(solve(convert_example(&example)), 7752969281844);
    }
}
