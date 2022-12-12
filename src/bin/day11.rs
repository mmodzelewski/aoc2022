use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Value {
    Old,
    Val(usize),
}

#[derive(Debug)]
enum Operation {
    Sum(usize),
    Multiplication(Value),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

fn main() {
    let input = std::fs::read_to_string("./src/input11.txt").unwrap();

    let mut monkeys: Vec<Monkey> = vec![];
    input
        .split("\n\n")
        .for_each(|monkey_input| {
            let lines = monkey_input.lines().collect::<Vec<_>>();
            let (_, items) = lines[1].split_once(": ").unwrap();
            let items: VecDeque<usize> = items.split(", ").filter_map(|item| item.parse::<usize>().ok()).collect();
            let (_, operation) = lines[2].split_once("= ").unwrap();
            let operation: Operation = if operation.contains("*") {
                let (_, val) = operation.split_once(" * ").unwrap();
                if val == "old" {
                    Operation::Multiplication(Value::Old)
                } else {
                    Operation::Multiplication(Value::Val(val.parse().unwrap()))
                }
            } else {
                let (_, val) = operation.split_once(" + ").unwrap();
                Operation::Sum(val.parse().unwrap())
            };
            let divisible_by = lines[3].split(" ").last().unwrap().parse::<usize>().unwrap();

            let if_true = lines[4].split(" ").last().unwrap().parse::<usize>().unwrap();
            let if_false = lines[5].split(" ").last().unwrap().parse::<usize>().unwrap();
            let monkey = Monkey {
                items,
                operation,
                divisible_by,
                if_true,
                if_false,
            };
            monkeys.push(monkey);
        });

    pt1(&mut monkeys);
}

fn pt1(monkeys: &mut Vec<Monkey>) {
    let divisors: usize = monkeys.iter().map(|monkey| monkey.divisible_by).product();
    let mut to_add: HashMap<usize, VecDeque<usize>> = HashMap::new();
    let mut operations_num = vec![0; monkeys.len()];
    for round in 0..10_000usize {
        // println!("Round {:?}", round + 1);
        for turn in 0..monkeys.len() {
            let monkey = &mut monkeys[turn];
            let items = &mut monkey.items;
            if let Some(value) = to_add.get_mut(&turn) {
                items.append(value);
            }

            // println!("Round {}, {:?}", round + 1, items);
            for _ in 0..items.len() {
                operations_num[turn] += 1;
                let item = items.pop_front().unwrap();
                // println!("{:?}", item);
                let mut worry_level = match &monkey.operation {
                    Operation::Sum(val) => {
                        item + val
                    }
                    Operation::Multiplication(val) => match val {
                        Value::Old => item.pow(2),
                        Value::Val(val) => item * val,
                    }
                };
                worry_level = worry_level % divisors;
                if worry_level % monkey.divisible_by as usize == 0 {
                    let key = monkey.if_true;
                    if let Some(value) = to_add.get_mut(&key) {
                        value.push_back(worry_level);
                    } else {
                        let mut value = VecDeque::new();
                        value.push_back(worry_level);
                        to_add.insert(key, value);
                    }
                } else {
                    let key = monkey.if_false;
                    if let Some(value) = to_add.get_mut(&key) {
                        value.push_back(worry_level);
                    } else {
                        let mut value = VecDeque::new();
                        value.push_back(worry_level);
                        to_add.insert(key, value);
                    }
                }
            }
        }
    }

    println!("{:?}", operations_num);
    operations_num.sort();
    operations_num.reverse();
    println!("{:?}", operations_num[0] as u64 * operations_num[1] as u64);
}
