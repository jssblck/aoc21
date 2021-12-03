use itertools::Itertools;

#[derive(Default)]
struct BasicPos {
    horizontal: i32,
    depth: i32,
}

impl BasicPos {
    fn solve(&self) -> i32 {
        self.horizontal * self.depth
    }
}

pub fn part_1(directions: &str) -> i32 {
    directions
        .lines()
        .map(|l| l.splitn(2, ' ').collect_tuple::<(&str, &str)>().unwrap())
        .map(|(command, steps)| (command, steps.parse::<i32>().unwrap()))
        .fold(
            BasicPos::default(),
            |BasicPos { depth, horizontal }, (command, steps)| match command {
                "forward" => BasicPos {
                    depth,
                    horizontal: horizontal + steps,
                },
                "down" => BasicPos {
                    depth: depth + steps,
                    horizontal,
                },
                "up" => BasicPos {
                    depth: depth - steps,
                    horizontal,
                },
                _ => panic!("unsupported command: '{}'", command),
            },
        )
        .solve()
}

#[derive(Default)]
struct AdvancedPos {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl AdvancedPos {
    fn solve(&self) -> i32 {
        self.horizontal * self.depth
    }
}

pub fn part_2(directions: &str) -> i32 {
    directions
        .lines()
        .map(|l| l.splitn(2, ' ').collect_tuple::<(&str, &str)>().unwrap())
        .map(|(command, steps)| (command, steps.parse::<i32>().unwrap()))
        .fold(
            AdvancedPos::default(),
            |pos, (command, steps)| match command {
                "forward" => AdvancedPos {
                    depth: pos.depth + (steps * pos.aim),
                    horizontal: pos.horizontal + steps,
                    ..pos
                },
                "down" => AdvancedPos {
                    aim: pos.aim + steps,
                    ..pos
                },
                "up" => AdvancedPos {
                    aim: pos.aim - steps,
                    ..pos
                },
                _ => panic!("unsupported command: '{}'", command),
            },
        )
        .solve()
}
