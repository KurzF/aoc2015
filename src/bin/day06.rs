use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Command {
    Toggle,
    Set(bool),
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    area: Rectangle,
    command: Command,
}

fn main() {
    let input = include_str!("../../input/day06.txt");

    let re = Regex::new(r#"([a-z\x20]+)(\d+),(\d+) through (\d+),(\d+)"#).unwrap();

    let instruction: Vec<_> = input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();

            let command = match caps[1].trim() {
                "turn off" => Command::Set(false),
                "turn on" => Command::Set(true),
                "toggle" => Command::Toggle,
                _ => unreachable!(),
            };

            let x1 = caps[2].parse::<usize>().unwrap();
            let y1 = caps[3].parse::<usize>().unwrap();

            let x2 = caps[4].parse::<usize>().unwrap();
            let y2 = caps[5].parse::<usize>().unwrap();

            let area = Rectangle { x1, y1, x2, y2 };

            Instruction { area, command }
        })
        .collect();

    println!(
        "Day 06 part one {:?} part two {:?}",
        part_one(&instruction),
        part_two(&instruction)
    );
}

fn part_one(instructions: &[Instruction]) -> usize {
    let mut grid = [false; 1_000_000];

    for inst in instructions {
        let Rectangle { x1, x2, y1, y2 } = inst.area;
        let command = inst.command;
        for x in x1..=x2 {
            for y in y1..=y2 {
                grid[x + y * 1000] = match command {
                    Command::Toggle => !grid[x + y * 1000],
                    Command::Set(v) => v,
                };
            }
        }
    }

    grid.iter().filter(|l| **l).count()
}

fn part_two(instructions: &[Instruction]) -> usize {
    let mut grid = [0; 1_000_000];

    for inst in instructions {
        let Rectangle { x1, x2, y1, y2 } = inst.area;
        let command = inst.command;
        for x in x1..=x2 {
            for y in y1..=y2 {
                let current_brightness = grid[x + y * 1000] as i32;

                grid[x + y * 1000] = match command {
                    Command::Toggle => (current_brightness + 2) as usize,
                    Command::Set(true) => (current_brightness + 1) as usize,
                    Command::Set(false) => std::cmp::max(0, current_brightness - 1) as usize,
                };
            }
        }
    }

    grid.iter().sum()
}
