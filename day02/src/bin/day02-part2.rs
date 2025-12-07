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

fn repeat_digits(digits: usize, repeat_count: usize) -> usize {
    if digits == 0 {
        return 0;
    }
    let len = digits.ilog10() + 1;
    let mut ret = 0;
    for i in 0..repeat_count {
        let pow = 10usize.pow((i as u32) * len);
        ret += digits * pow;
    }
    ret
}

fn valid_id(id: usize) -> bool {
    let len: usize = (id.ilog10() + 1) as usize;
    for i in 1..=(len / 2) {
        let slice = id % 10usize.pow(i as u32);
        if len % i != 0 {
            continue;
        }
        let repeat = repeat_digits(slice, len / i);
        if id == repeat {
            return false;
        }
    }
    true
}

fn solve(ranges: Vec<&str>) -> usize {
    let mut ret = 0;
    for range in ranges {
        let (start, end) = range.split_once('-').unwrap();
        let u_start = start.parse::<usize>().unwrap();
        let u_end = end.parse::<usize>().unwrap();
        ret += (u_start..=u_end)
            .into_par_iter()
            .filter(|id| !valid_id(*id))
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

    let mut reader = BufReader::new(opt.input);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let ranges: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
    println!("{}", solve(ranges));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn given() {
        let example = Vec::from([
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
        ]);
        println!("{}", example.join(","));
        assert_eq!(solve(example), 4174379265);
    }
}
