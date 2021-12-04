use aoc21::*;

macro_rules! day {
    ($num:literal, $fn:tt) => {
        println!("day {}:\n", $num);
        $fn();
        println!("\n----");
    };
}

fn main() {
    day!("1", day1);
    day!("2", day2);
}

fn day1() {
    let input = include_str!("./inputs/day1.txt");
    println!("part_1: {}", day1::part_1(input));
    println!("part_2: {}", day1::part_2(input));
}

fn day2() {
    let steps = include_str!("./inputs/day2.txt");
    println!("part_1: {}", day2::part_1(steps));
    println!("part_2: {}", day2::part_2(steps));
}
