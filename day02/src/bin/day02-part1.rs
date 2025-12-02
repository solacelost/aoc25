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

fn valid_id(id: &usize) -> bool {
    let string = id.to_string();
    let len = string.len();
    if len % 2 == 1 {
        return true;
    }
    let (l, r) = string.split_at(len / 2);
    if l == r {
        return false;
    }
    return true;
}

fn solve(lines: Vec<String>) -> usize {
    let mut ret = 0;
    for range in lines {
        let (start, end) = range.split_once('-').unwrap();
        let u_start = start.parse::<usize>().unwrap();
        let u_end = end.parse::<usize>().unwrap();
        ret += (u_start..(u_end + 1))
            .into_par_iter()
            .filter(|id| !valid_id(id))
            .sum::<usize>();
    }
    ret
}

fn main() -> io::Result<()> {
    let opt = Opt::parse();

    rayon::ThreadPoolBuilder::new()
        .num_threads(opt.threads)
        .build_global()
        .unwrap();

    let reader = BufReader::new(opt.input);
    let lines: Vec<String> = reader
        .lines()
        .flatten()
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(',')
                .map(|st| st.to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect();
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
            "11-22",
            "95-115",
            "998-1012",
            "1188511880-1188511890",
            "222220-222224",
            "1698522-1698528",
            "446443-446449",
            "38593856-38593862",
            "565653-565659",
            "824824821-824824827",
            "2121212118-2121212124",
        ];
        println!("{}", example.join(","));
        assert_eq!(solve(convert_example(&example)), 1227775554);
    }
}
