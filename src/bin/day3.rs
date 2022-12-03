fn get_example_input() -> &'static str {
    "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
}

fn main() {
    let result = part1();
    println!("{:?}", result);

    let result2 = part2();
    println!("{:?}", result2);
}

fn part1() -> i32 {
    let input = std::fs::read_to_string("./src/input3.txt").unwrap();
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(a, b)| a.chars().find(|c| b.contains(*c)))
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .map(|c| get_priority(c))
        .sum()
}

fn part2() -> i32 {
    //let input = get_example_input();
    let input = std::fs::read_to_string("./src/input3.txt").unwrap();
    let mut iter = input.lines();
    let mut res = 0;

    loop {
        let line1_opt = iter.next();
        if line1_opt.is_none() {
            break;
        }
        let line1 = line1_opt.unwrap();
        let line2 = iter.next().unwrap();
        let line3 = iter.next().unwrap();

        let character = line1
            .chars()
            .filter(|c| line2.contains(*c))
            .filter(|c| line3.contains(*c))
            .next();
        if let Some(c) = character {
            res += get_priority(c)
        }
    }
    return res;
}

fn get_priority(character: char) -> i32 {
    let c = character as i32;
    if c > 96 {
        c - 96
    } else {
        c - 38
    }
}
