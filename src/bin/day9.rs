use std::collections::HashSet;

fn get_example_input() -> &'static str {
    "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"
}
fn get_example_input_2() -> &'static str {
    "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
}

fn main() {
    // let input = get_example_input();
    let input = std::fs::read_to_string("./src/input9.txt").unwrap();
    pt1(&input);
    pt2(&input);
}

fn pt2(input: &str) {
    let mut positions = vec![(500usize, 500usize); 10];
    let mut tail_positions: HashSet<(usize, usize)> = HashSet::new();

    input.lines().for_each(|line| {
        let (dir, count) = line.split_once(" ").unwrap();
        let count = count.parse::<usize>().unwrap();
        for _i in 0..count {
            // print_rope(&positions);
            let head_pos = positions.first_mut().unwrap();
            match dir {
                "U" => head_pos.1 += 1,
                "D" => head_pos.1 -= 1,
                "L" => head_pos.0 -= 1,
                "R" => head_pos.0 += 1,
                _ => panic!("unknown direction"),
            }
            for i in 1..10 {
                let head = positions[i - 1];
                let following = positions[i];
                positions[i] = calculate_tail_pos(head, following);
            }
            tail_positions.insert(*positions.last().unwrap());
            // println!("{} {}", line, _i);
        }
    });

    println!("{:?}", positions.first().unwrap());
    println!("{:?}", tail_positions.len());
    // println!("{:?}", tail_positions);
}

fn print_rope(positions: &Vec<(usize, usize)>) {
    println!("");
    for j in (490..520).rev() {
        for i in 490..520 {
            let pos = positions.iter().position(|&pos| pos == (i, j));
            match pos {
                Some(idx) => print!("{}", idx),
                None => print!("."),
            }
        }
        println!("");
    }
    println!("****************************************");
}

fn pt1(input: &str) {
    let mut head_pos = (500usize, 500usize);
    let mut tail_pos = (500usize, 500usize);
    let mut tail_positions: HashSet<(usize, usize)> = HashSet::new();

    input.lines().for_each(|line| {
        let (dir, count) = line.split_once(" ").unwrap();
        let count = count.parse::<usize>().unwrap();
        for _i in 0..count {
            match dir {
                "U" => head_pos.1 += 1,
                "D" => head_pos.1 -= 1,
                "L" => head_pos.0 -= 1,
                "R" => head_pos.0 += 1,
                _ => panic!("unknown direction"),
            }
            tail_pos = calculate_tail_pos(head_pos, tail_pos);
            tail_positions.insert(tail_pos);
        }
    });

    println!("{:?}", head_pos);
    println!("{:?}", tail_positions.len());
}

fn calculate_tail_pos(head_pos: (usize, usize), tail_pos: (usize, usize)) -> (usize, usize) {
    if head_pos.0.abs_diff(tail_pos.0) <= 1 && head_pos.1.abs_diff(tail_pos.1) <= 1 {
        return tail_pos;
    }
    if head_pos.0.abs_diff(tail_pos.0) > 1 && head_pos.1.abs_diff(tail_pos.1) > 1 {
        let x = if head_pos.0 > tail_pos.0 {
            tail_pos.0 + 1
        } else {
            tail_pos.0 - 1
        };
        let y = if head_pos.1 > tail_pos.1 {
            tail_pos.1 + 1
        } else {
            tail_pos.1 - 1
        };
        return (x, y);
    }
    if head_pos.0.abs_diff(tail_pos.0) > 1 {
        if head_pos.1 == tail_pos.1 {
            if head_pos.0 < tail_pos.0 {
                return (tail_pos.0 - 1, tail_pos.1);
            } else {
                return (tail_pos.0 + 1, tail_pos.1);
            }
        } else {
            if head_pos.0 < tail_pos.0 {
                return (tail_pos.0 - 1, tail_pos.1 + head_pos.1 - tail_pos.1);
            } else {
                return (tail_pos.0 + 1, tail_pos.1 + head_pos.1 - tail_pos.1);
            }
        }
    }
    if head_pos.1.abs_diff(tail_pos.1) > 1 {
        if head_pos.0 == tail_pos.0 {
            if head_pos.1 < tail_pos.1 {
                return (tail_pos.0, tail_pos.1 - 1);
            } else {
                return (tail_pos.0, tail_pos.1 + 1);
            }
        } else {
            if head_pos.1 < tail_pos.1 {
                return (tail_pos.0 + head_pos.0 - tail_pos.0, tail_pos.1 - 1);
            } else {
                return (tail_pos.0 + head_pos.0 - tail_pos.0, tail_pos.1 + 1);
            }
        }
    }
    panic!("all cases should be covered");
}
