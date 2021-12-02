use anyhow::Result;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let lines: Vec<_> = input.lines().collect();

    part1(&lines);
    part2(&lines);
    Ok(())
}

fn part1(lines: &Vec<&str>) {
    let mut position = vec![0, 0];
    for line in lines {
        let command: Vec<_> = line.split_whitespace().collect();
        let direction = command[0];
        let magnitude = command[1].parse::<i32>().unwrap();

        match direction {
            "forward" => position[0] += magnitude,
            "up" => position[1] -= magnitude,
            "down" => position[1] += magnitude,
            _ => (),
        }
    }

    println!("{}", position[0] * position[1]);
}

struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

fn part2(lines: &Vec<&str>) {
    let mut position = Position { x: 0, y: 0, aim: 0 };
    for line in lines {
        let command: Vec<_> = line.split_whitespace().collect();
        let direction = command[0];
        let magnitude = command[1].parse::<i32>().unwrap();

        match direction {
            "forward" => {
                position.x += magnitude;
                position.y += position.aim * magnitude;
            }
            "up" => position.aim -= magnitude,
            "down" => position.aim += magnitude,
            _ => (),
        }
    }

    println!("{}", position.x * position.y);
}
