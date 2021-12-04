use anyhow::Result;
use num::pow;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let lines: Vec<_> = input.lines().collect();

    part1(&lines);
    Ok(())
}

fn to_binary(binary: &Vec<usize>) -> usize {
    let mut num = 0;
    for (i, n) in binary.iter().rev().enumerate() {
        num += n * pow(2, i);
    }

    num
}

fn part1(lines: &Vec<&str>) {
    let total = lines.len();
    let mut sums = vec![0; lines[0].len()];

    for line in lines {
        for (i, c) in line.chars().enumerate() {
            sums[i] += c.to_string().parse::<usize>().unwrap();
        }
    }

    let diagnostic: Vec<_> = sums
        .iter()
        .map(|n| if *n > total / 2 { 1 } else { 0 })
        .collect();
    let gamma: Vec<_> = diagnostic
        .iter()
        .map(|n| if *n == 0 { 1 } else { 0 })
        .collect();

    let output = to_binary(&diagnostic) * to_binary(&gamma);
    println!("{}", output);
}
