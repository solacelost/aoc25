use clap::Parser;
use clio::Input;
use std::io::{self, BufReader, prelude::*};

#[derive(Parser)]
struct Opt {
    /// Input file, use '-' for stdin
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

fn valid_id(id: usize) -> bool {
    let len = id.ilog10() + 1;
    if len % 2 == 1 {
        return true;
    }
    let middle_pow = 10usize.pow(len / 2);
    let (l, r) = (id / middle_pow, id % middle_pow);
    if l == r {
        return false;
    }
    return true;
}

fn solve(ranges: Vec<&str>) -> usize {
    let mut ret = 0;
    for range in ranges {
        let (start, end) = range.split_once('-').unwrap();
        let u_start = start.parse::<usize>().unwrap();
        let u_end = end.parse::<usize>().unwrap();
        ret += (u_start..=u_end)
            .into_iter()
            .filter(|id| !valid_id(*id))
            .sum::<usize>();
    }
    ret
}

fn main() -> io::Result<()> {
    let opt = Opt::parse();

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
        assert_eq!(solve(example), 1227775554);
    }
}
