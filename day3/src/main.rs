use std::fs;

use regex::Regex;
use anyhow::Result;

fn part_one(input: &String) -> Result<()> {
    let expr = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;

    let total: usize = expr.captures_iter(input)
        .map(|captures| captures[1].parse::<usize>().unwrap() * captures[2].parse::<usize>().unwrap())
        .sum();

    println!("total: {}", total);

    Ok(())
}

fn part_two(input: &String) -> Result<()> {
    let expr = Regex::new(r"(mul)\(([0-9]{1,3}),([0-9]{1,3})\)|(do)\(\)|(don't)\(\)")?;

    let mut enabled = true;
    let total: usize = expr.captures_iter(input)
        .fold(0, |acc, captures| {
            if captures.get(1).is_some() {
                if !enabled {
                    return acc;
                }

                let arg1 = captures[2].parse::<usize>().unwrap();
                let arg2 = captures[3].parse::<usize>().unwrap();

                return acc + arg1 * arg2;
            }
            
            if captures.get(4).is_some() {
                enabled = true;
            } else if captures.get(5).is_some() {
                enabled = false;
            }

            acc
        });

    println!("total with enabler: {}", total);

    Ok(())
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    part_one(&input)?;
    part_two(&input)?;

    Ok(())
}
