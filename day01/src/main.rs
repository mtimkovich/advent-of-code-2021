use anyhow::Result;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nums: Vec<u32> = input.lines().map(|s| s.parse().unwrap()).collect();

    part1(&nums);
    part2(&nums);
    Ok(())
}

fn part1(input: &Vec<u32>) {
    let mut increases = 0;
    for (i, depth) in input.iter().enumerate().skip(1) {
        if input[i - 1] < *depth {
            increases += 1;
        }
    }

    println!("{}", increases);
}

fn part2(input: &Vec<u32>) {
    let mut increases = 0;
    let mut prev = None;
    for (i, _) in input.iter().enumerate().skip(2) {
        let window = input[i] + input[i - 1] + input[i - 2];

        if let Some(prev) = prev {
            if prev < window {
                increases += 1;
            }
        }

        prev = Some(window);
    }

    println!("{}", increases);
}
