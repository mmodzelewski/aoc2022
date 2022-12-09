fn get_examle_input() -> &'static str {
    "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"
}

fn main() {
    // let input = get_examle_input();
    let input = std::fs::read_to_string("./src/input7.txt").unwrap();

    let mut acc: Vec<(&str, usize)> = vec![("/", 0)];
    let mut result: Vec<(&str, usize)> = vec![];

    input.lines().for_each(|line| {
        if line.starts_with("$ cd /") || line.starts_with("$ ls") || line.starts_with("dir ") {
            return;
        }

        if line.starts_with("$ cd ..") {
            let last = acc.pop().unwrap();
            result.push(last);
            acc.last_mut().unwrap().1 += last.1;
        } else if line.starts_with("$ cd ") {
            let name = &line[5..];
            acc.push((name, 0));
        } else {
            let (size, name) = line.split_once(" ").unwrap();
            let last = acc.last_mut().unwrap();
            last.1 += size.parse::<usize>().unwrap();
        }
    });

    while acc.len() > 0 {
        let last = acc.pop().unwrap();
        result.push(last);
        if acc.len() > 0 {
            acc.last_mut().unwrap().1 += last.1;
        }
    }

    let limit = 100000;
    let res: usize = result.iter().filter(|(_, size)| size < &limit).map(|(_, size)| size).sum();

    let total_space = 70000000;
    let required_space = 30000000;
    let unused_space = total_space - result.last().unwrap().1;
    let to_delete = required_space - unused_space;
    let res2 = result.iter().map(|(_, size)| size).filter(|&size| size >= &to_delete).min();

    println!("{:?}", res);
    println!("{:?}", res2);
}
