use std::{str::FromStr, string::ParseError};

use regex::{Captures, Regex};

fn get_example_input() -> &'static str {
    "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    fn new(input: &str) -> Self {
        let stacks_initial_vec = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stacks_initial_vec[0].len()];

        for line in stacks_initial_vec.iter() {
            for (idx, ch) in line.iter().enumerate() {
                stacks[idx].push(*ch);
            }
        }

        let stacks: Vec<Vec<char>> = stacks
            .iter()
            .filter(|stack| stack.last().unwrap().is_numeric())
            .map(|stack| {
                stack
                    .iter()
                    .filter(|ch| !ch.is_whitespace())
                    .map(|x| *x)
                    .rev()
                    .collect::<Vec<char>>()
            })
            .collect();

        Self { stacks }
    }

    fn get(&mut self, idx: usize) -> &mut Vec<char> {
        return &mut self.stacks[idx];
    }

    fn to_string(&self) -> String {
        let top_crates = self
            .stacks
            .iter()
            .map(|stack| *stack.last().unwrap() as u8)
            .collect::<Vec<u8>>();
        std::str::from_utf8(&top_crates).unwrap().to_string()
    }

    fn mv1(&mut self, mv: &Move) {
        for _ in 0..mv.count {
            let from = self.get(mv.from);
            let value = from.pop();
            let to = self.get(mv.to);
            to.push(value.unwrap());
        }
    }

    fn mv2(&mut self, mv: &Move) {
        let from = self.get(mv.from);
        let mut temp: Vec<char> = Vec::with_capacity(mv.count);
        for _ in 0..mv.count {
            temp.insert(0, from.pop().unwrap());
        }
        let to = self.get(mv.to);
        to.append(&mut temp);
    }
}

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let pattern: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let captured = pattern.captures(input).unwrap();
        Ok(Move {
            count: capture_to_number(&captured, 1),
            from: capture_to_number(&captured, 2) - 1,
            to: capture_to_number(&captured, 3) - 1,
        })
    }
}

fn capture_to_number(captures: &Captures, idx: usize) -> usize {
    captures.get(idx).unwrap().as_str().parse().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("./src/input5.txt").unwrap();
    let (stacks, moves) = input.split_once("\n\n").unwrap();
    let moves = moves
        .lines()
        .filter_map(|line| line.parse::<Move>().ok())
        .collect::<Vec<_>>();
    pt1(stacks, &moves);
    pt2(stacks, &moves);
}

fn pt1(stacks: &str, moves: &Vec<Move>) {
    let mut stacks = Stacks::new(stacks);

    moves.iter().for_each(|mv| stacks.mv1(&mv));

    println!("{}", stacks.to_string());
}

fn pt2(stacks: &str, moves: &Vec<Move>) {
    let mut stacks = Stacks::new(stacks);

    moves.iter().for_each(|mv| stacks.mv2(&mv));

    println!("{}", stacks.to_string());
}
