use day2::*;

fn main() {
    let steps = include_str!("./part_1.txt");

    let result = part_1(steps);
    println!("part_1: {}", result);

    let result = part_2(steps);
    println!("part_2: {}", result);
}
