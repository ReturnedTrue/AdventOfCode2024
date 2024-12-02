use std::fs;
use anyhow::Result;

fn part_one(inputs: &Vec<Vec<isize>>) {
    let one_to_three = 1..=3;

    let safe: usize = inputs
        .iter()
        .map(|inner_inputs| {
            let as_diffs: Vec<isize> = (1..inner_inputs.len())
                .map(|i| inner_inputs[i] - inner_inputs[i - 1])
                .collect();

            println!("diffs: {:?}", as_diffs);

            as_diffs
                .iter()
                .all(|diff| diff.signum() == as_diffs[0].signum() && one_to_three.contains(&diff.abs()))
        })
        .filter(|b| *b)
        .count();

    println!("no. safe: {}", safe);
}

fn main() -> Result<()> {
    let inputs = fs::read_to_string("inputs.txt")?;
    let inputs: Vec<Vec<isize>> = inputs
        .split("\n")
        .map(|input| input.trim_end().split(" "))
        .map(|split_input| split_input.map(|n| n.parse().unwrap()).collect())
        .collect();

    part_one(&inputs);

    Ok(())
}
