use std::fs::read_to_string;

fn get_example_input() -> &'static str {
    "30373
25512
65332
33549
35390"
}

#[derive(Debug)]
struct Tree {
    height: usize,
    visible: bool,
    scenic_score: usize,
}

fn main() {
    let input = read_to_string("./src/input8.txt").unwrap();
    // let input = get_example_input();
    let mut forest = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|ch| !ch.is_empty())
                .map(|ch| ch.parse::<usize>().unwrap())
                .map(|height| Tree {
                    height,
                    visible: false,
                    scenic_score: 0,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    calculate_scenic_score(&mut forest);
    let best_scenic_view = forest
        .iter()
        .flatten()
        .map(|tree| tree.scenic_score)
        .max()
        .unwrap();
    println!("Scenic view {}", best_scenic_view);

    mark_trees(&mut forest);

    forest.iter_mut().for_each(|row| row.reverse());
    forest.reverse();

    mark_trees(&mut forest);
    let result = forest.iter().flatten().filter(|tree| tree.visible).count();

    println!("Visible trees {:?}", result);
}

fn calculate_scenic_score(forest: &mut Vec<Vec<Tree>>) {
    for j in 0..forest.len() {
        for i in 0..forest[j].len() {
            if i == 0 || i == forest.len() - 1 || j == 0 || j == forest.len() - 1 {
                continue;
            }
            let score = calculate_scenic_score_for_tree(i, j, forest);
            forest[j][i].scenic_score = score;
        }
    }
}

fn calculate_scenic_score_for_tree(x: usize, y: usize, forest: &Vec<Vec<Tree>>) -> usize {
    let tree = &forest[y][x];

    // look up
    let mut i = x;
    let mut j = y;
    let mut up = 0;
    while j > 0 {
        j -= 1;
        up += 1;
        if forest[j][i].height >= tree.height {
            break;
        }
    }
    // down
    let mut down = 0;
    i = x;
    j = y;
    while j < forest.len() - 1 {
        j += 1;
        down += 1;
        if forest[j][i].height >= tree.height {
            break;
        }
    }
    // left
    let mut left = 0;
    i = x;
    j = y;
    while i > 0 {
        i -= 1;
        left += 1;
        if forest[j][i].height >= tree.height {
            break;
        }
    }
    // right
    let mut right = 0;
    i = x;
    j = y;
    while i < forest.len() - 1 {
        i += 1;
        right += 1;
        if forest[j][i].height >= tree.height {
            break;
        }
    }
    return up * down * left * right;
}

fn mark_trees(forest: &mut Vec<Vec<Tree>>) {
    let mut heighest_top = vec![0; forest.len()];
    let mut heighest_left;
    for j in 0..forest.len() {
        heighest_left = forest[j][0].height;
        for i in 0..forest[j].len() {
            // println!("i {}, j {}", i, j);
            if j == 0 {
                heighest_top[i] = forest[0][i].height;
            }
            if i == 0 || j == 0 {
                forest[j][i].visible = true;
                continue;
            }

            let tree = &mut forest[j][i];
            if tree.height > heighest_top[i] {
                tree.visible = true;
                heighest_top[i] = tree.height;
            }
            if tree.height > heighest_left {
                tree.visible = true;
                heighest_left = tree.height;
            }
        }
    }
}
