fn main() {
    let input = std::fs::read_to_string("./src/input10.txt").unwrap();

    let mut cycle = 0usize;
    let mut state = 1isize;
    let mut to_add: Option<isize> = None;
    let mut commands = input.lines();
    let mut cycle_to_print = 20usize;
    let mut result = 0isize;

    loop {
        cycle += 1;

        // println!("cycle {}, state {}", cycle, state);
        if cycle == cycle_to_print {
            let strength = cycle as isize * state;
            // println!("cycle {}, state {}, strength {}", cycle, state, strength);
            result += strength;
            cycle_to_print += 40;
        }

        let print_pos: isize = ((cycle - 1) % 40) as isize;
        if print_pos >= state - 1 && print_pos <= state + 1 {
            print!("#");
        } else {
            print!(".");
        }
        if print_pos == 39 {
            println!();
        }

        if let Some(val) = to_add {
            state += val;
            to_add = None;
            continue;
        }

        let command = commands.next();
        if command.is_none() {
            break;
        }

        let command = command.unwrap();
        if command == "noop" {
            continue;
        }

        let (_, value) = command.split_once(" ").unwrap();
        let value = value.parse::<isize>().unwrap();
        to_add = Some(value);
    }

    println!("result {}", result);
}
