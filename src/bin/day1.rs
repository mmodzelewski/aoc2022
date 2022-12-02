use aoc2022::day1_input::get_input;

fn main() {
    let result = part1();
    println!("{:?}", result);

    let result2 = part2();
    println!("{:?}", result2);
}

fn part1() -> i32 {
    get_input()
        .split("\n\n")
        .map(|items| {
            items
                .lines()
                .map(|item| item.parse::<i32>().expect("This must be a number"))
                .fold(0, |a, b| a + b)
        })
        .max()
        .unwrap()
}

fn part2() -> i32 {
    let mut result: Vec<i32> = get_input()
        .split("\n\n")
        .map(|items| {
            items
                .lines()
                .map(|item| item.parse::<i32>().expect("This must be a number"))
                .fold(0, |a, b| a + b)
        })
        .collect();

    result.sort_by(|a, b| b.cmp(a));
    result.truncate(3);

    return result.iter().sum();
}
