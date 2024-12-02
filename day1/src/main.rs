use std::fs;
use anyhow::Result;

fn part_one(left_vec: &Vec<usize>, right_vec: &Vec<usize>, inputs_count: usize) -> Result<()> {
    let total_distance: usize = (0..inputs_count)
        .map(|i| (left_vec[i], right_vec[i]))
        .inspect(|(l, r)| println!("{} | {}", l, r))
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    println!("difference: {}", total_distance);

    Ok(())
}

fn part_two(left_vec: &Vec<usize>, right_vec: &Vec<usize>, _inputs_count: usize) -> Result<()> {
    let similarity: usize = left_vec
        .iter()
        .map(|l| l * right_vec.iter().filter(|r| l == *r).count())
        .sum();

    println!("similarity: {}", similarity);

    Ok(())
}

fn main() -> Result<()> {
    let inputs = fs::read_to_string("inputs.txt")?;
    let inputs: Vec<&str> = inputs.split("\n").collect();

    let inputs_count = inputs.len();

    let mut left_vec: Vec<usize> = Vec::with_capacity(inputs_count);
    let mut right_vec: Vec<usize> = Vec::with_capacity(inputs_count);

    for input in inputs {
        let split_input: Vec<&str> = input.trim_end().split("   ").collect();

        left_vec.push(split_input.get(0).unwrap().parse()?);
        right_vec.push(split_input.get(1).unwrap().parse()?);
    }

    left_vec.sort();
    right_vec.sort();

    part_one(&left_vec, &right_vec, inputs_count)?;
    part_two(&left_vec, &right_vec, inputs_count)?;

    Ok(())
}
