use day1::*;

macro_rules! time {
    ($s:stmt) => {
        let timer = std::time::Instant::now();
        $s
        println!("timing: {:?}", timer.elapsed());
    };
}

fn main() {
    let input = include_str!("./part_1.txt");
    time!(println!("preloaded {} bytes", input.len()));

    time!(println!("part_1: {}", part_1(input)));
    time!(println!("part_2: {}", part_2(input)));

    time!(println!("part_1_mut: {}", part_1_no_mutate(input)));
    time!(println!("part_1_no_option: {}", part_1_no_option(input)));
    time!(println!("part_1_opt_pat: {}", part_1_opt_pat(input)));
}
