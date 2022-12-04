use std::{num::ParseIntError, str::FromStr};

fn get_example_input() -> &'static str {
    "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
}

#[derive(Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, e) = s.split_once("-").unwrap();
        let start: u32 = str::parse(s).unwrap();
        let end: u32 = str::parse(e).unwrap();
        return Ok(Range { start, end });
    }
}

fn main() {
    let result = part1();
    println!("{:?}", result);

    let result2 = part2();
    println!("{:?}", result2);
}

fn part1() -> usize {
    let input = std::fs::read_to_string("./src/input4.txt").unwrap();
    // let input = get_example_input();
    input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| {
            let a_range = a.parse::<Range>().unwrap();
            let b_range = b.parse::<Range>().unwrap();
            let res = a_range.start <= b_range.start && a_range.end >= b_range.end
                || b_range.start <= a_range.start && b_range.end >= a_range.end;
            return res;
        })
        .filter(|res| *res)
        .count()
}

fn part2() -> usize {
    let input = std::fs::read_to_string("./src/input4.txt").unwrap();
    // let input = get_example_input();
    input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(a, b)| {
            let a_range = a.parse::<Range>().unwrap();
            let b_range = b.parse::<Range>().unwrap();
            let res = a_range.start <= b_range.end && a_range.end >= b_range.start;
            return res;
        })
        .filter(|res| *res)
        .count()
}
