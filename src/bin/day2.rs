use std::collections::HashMap;

fn get_example_input() -> &'static str {
    "A Y
B X
C Z"
}

fn main() {
    let result = part1();
    println!("{:?}", result);

    let result2 = part2();
    println!("{:?}", result2);
}

fn part1() -> i32 {
    let input = std::fs::read_to_string("./src/input2.txt").unwrap();
    let map = get_map_1();
    input
        .lines()
        .map(|line| map.get(line))
        .map(|value| value.unwrap_or(&0))
        .sum()
}

fn get_map_1() -> HashMap<&'static str, i32> {
    HashMap::from([
        ("A X", 1 + 3),
        ("A Y", 2 + 6),
        ("A Z", 3 + 0),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 1 + 6),
        ("C Y", 2 + 0),
        ("C Z", 3 + 3),
    ])
}

fn get_map_2() -> HashMap<&'static str, i32> {
    HashMap::from([
        ("A X", 3 + 0),
        ("A Y", 1 + 3),
        ("A Z", 2 + 6),
        ("B X", 1 + 0),
        ("B Y", 2 + 3),
        ("B Z", 3 + 6),
        ("C X", 2 + 0),
        ("C Y", 3 + 3),
        ("C Z", 1 + 6),
    ])
}

fn part2() -> i32 {
    // let input = get_example_input();
    let input = std::fs::read_to_string("./src/input2.txt").unwrap();
    let map = get_map_2();

    input
        .lines()
        .map(|line| map.get(line))
        .map(|value| value.unwrap_or(&0))
        .sum()
}
