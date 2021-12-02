pub fn part_1(measurements: &str) -> i32 {
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

pub fn part_2(measurements: &str) -> i32 {
    let measurements = measurements
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

pub fn part_1_no_mutate(measurements: &str) -> i32 {
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

pub fn part_1_no_option(measurements: &str) -> i32 {
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

pub fn part_1_opt_pat(measurements: &str) -> i32 {
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
