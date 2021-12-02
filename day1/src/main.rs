macro_rules! time {
    ($s:stmt) => {
        let timer = std::time::Instant::now();
        $s
        println!("timing: {:?}", timer.elapsed());
    };
}

fn main() {
    let s = include_str!("./part_1.txt");
    time!(println!("preloaded {} bytes", s.len()));

    time!(println!("part_1: {}", part_1()));
    time!(println!("part_2: {}", part_2()));

    time!(println!("part_1_mut: {}", part_1_no_mutate()));
    time!(println!("part_1_no_option: {}", part_1_no_option()));
    time!(println!("part_1_opt_pat: {}", part_1_opt_pat()));
}

fn part_1() -> i32 {
    let measurements = include_str!("./part_1.txt");

    let mut prev_value = None;
    let mut increases = 0;
    for line in measurements.lines() {
        let current_value = line.parse::<i32>().unwrap();
        if let Some(p) = prev_value {
            if p < current_value {
                increases += 1;
            }
        }
        prev_value = Some(current_value);
    }

    increases
}

fn part_2() -> i32 {
    let measurements = include_str!("./part_1.txt")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut prev_value = None;
    let mut increases = 0;
    for window in measurements.windows(3) {
        let current_value = window.iter().sum::<i32>();
        if let Some(p) = prev_value {
            if p < current_value {
                increases += 1;
            }
        }
        prev_value = Some(current_value);
    }

    increases
}

fn part_1_no_mutate() -> i32 {
    let measurements = include_str!("./part_1.txt");

    measurements
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .fold((0, None), |(increases, prev), current| {
            let increases = prev.map_or(increases, |prev| {
                if current > prev {
                    increases + 1
                } else {
                    increases
                }
            });
            (increases, Some(current))
        })
        .0
}

fn part_1_no_option() -> i32 {
    let measurements = include_str!("./part_1.txt");

    measurements
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .fold((0, -1), |(increases, prev), current| {
            let increases = if prev > -1 && current > prev {
                increases + 1
            } else {
                increases
            };
            (increases, current)
        })
        .0
}

fn part_1_opt_pat() -> i32 {
    let measurements = include_str!("./part_1.txt");

    measurements
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .fold((0, None), |(increases, prev), current| {
            let increases = match prev {
                Some(prev) => {
                    if current > prev {
                        increases + 1
                    } else {
                        increases
                    }
                }
                None => increases,
            };
            (increases, Some(current))
        })
        .0
}
