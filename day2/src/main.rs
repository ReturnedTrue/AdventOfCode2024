use std::fs;
use anyhow::Result;

fn part_one(inputs: &Vec<Vec<isize>>) {
    let one_to_three = 1..=3;

    let safe: usize = inputs
        .iter()
		.cloned()
        .filter(|inner_inputs| {
            let as_diffs: Vec<isize> = (1..inner_inputs.len())
                .map(|i| inner_inputs[i] - inner_inputs[i - 1])
                .collect();

            as_diffs
                .iter()
                .all(|diff| diff.signum() == as_diffs[0].signum() && one_to_three.contains(&diff.abs()))
        })
        .count();

    println!("no. safe: {}", safe);
}

fn part_two(inputs: &Vec<Vec<isize>>) {
    let one_to_three = 1..=3;

    let safe: usize = inputs
        .iter()
		.cloned()
		// each line in the input, ie. 2 3 4 6 5
        .filter(|input_line| {
			// from 0 to n
			(0..input_line.len())
				// map each index to a count of how many pass the levels check
				.map(|i| {
					let mut line_without_one_level = input_line.clone();
					line_without_one_level.remove(i);

					let as_diffs: Vec<isize> = (1..line_without_one_level.len())
						.map(|i| line_without_one_level[i] - line_without_one_level[i - 1])
						.collect();
		
					as_diffs
						.iter()
						.filter(|diff| diff.signum() == as_diffs[0].signum() && one_to_three.contains(&diff.abs()))
						.count()
				})
				// find the maximum of how many pass the levels check
				.max()
				// ensure this maximum is only because of one bad level being removed
				.unwrap() >= input_line.len() - 2

        })
		// count how many lines pass this 
        .count();

    println!("no. safe after problem dampener: {}", safe);
}

fn main() -> Result<()> {
    let inputs = fs::read_to_string("inputs.txt")?;
    let inputs: Vec<Vec<isize>> = inputs
        .split("\n")
        .map(|input| input.trim_end().split(" "))
        .map(|split_input| split_input.map(|n| n.parse().unwrap()).collect())
        .collect();

    part_one(&inputs);
	part_two(&inputs);

    Ok(())
}
