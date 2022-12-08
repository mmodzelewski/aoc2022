use std::collections::HashSet;

fn get_example_input() -> &'static str {
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
}

fn main() {
    let input = std::fs::read_to_string("./src/input6.txt").unwrap();
    // let input = get_example_input();

    let mut vec: Vec<char> = Vec::new();
    for (idx, ch) in input.char_indices() {
        vec.push(ch);
        if vec.len() > 14 {
            vec.remove(0);
        }
        if vec.len() == 14 {
            let mut set: HashSet<char> = HashSet::new();
            vec.iter().for_each(|ch| {
                set.insert(*ch);
            });
            if set.len() == 14 {
                println!("{:?} {:?}", vec, idx + 1);
                break;
            }
        }
    }
}
