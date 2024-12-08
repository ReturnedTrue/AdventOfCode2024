use std::fs;

use anyhow::Result;

fn parse_num(s: &str) -> usize {
	s.parse().unwrap()
}

enum OperatorPart1 {
	Add,
	Mul
}

fn view_node_part1(index: usize, nodes: &Vec<usize>, total: usize, target: &usize, operator: OperatorPart1) -> bool {
	let total = match operator {
		OperatorPart1::Add => total + nodes[index],
		OperatorPart1::Mul => total * nodes[index]
	};

	let index = index + 1;

	if index == nodes.len() {
		return total == *target;
	}

	return view_node_part1(index, nodes, total, target, OperatorPart1::Add) || view_node_part1(index, nodes, total, target, OperatorPart1::Mul);
}

fn part_one(input: &String) {
    let total_calibration = input
		.split("\n")
		.map(|line| {
			let mut parts = line.trim().split(":");

			let target = parse_num(parts.next().unwrap());
			let nums: Vec<usize> = parts.next().unwrap().trim().split(" ").map(parse_num).collect();

			(target, nums)
		})
		.filter(|(target, nums)| view_node_part1(0, &nums, 0, target, OperatorPart1::Add))
		.fold(0, |acc, (target, _nums)| acc + target);

	println!("Total calibration: {}", total_calibration);
}

enum Operator {
	Add,
	Mul,
	Concat
}

fn view_node(index: usize, nodes: &Vec<usize>, total: usize, target: &usize, operator: Operator) -> bool {
	let total = match operator {
		Operator::Add => total + nodes[index],
		Operator::Mul => total * nodes[index],
		Operator::Concat => parse_num(&(total.to_string() + &nodes[index].to_string()))
	};

	let index = index + 1;

	if index == nodes.len() {
		return total == *target;
	}

	return view_node(index, nodes, total, target, Operator::Add) 
		|| view_node(index, nodes, total, target, Operator::Mul) 
		|| view_node(index, nodes, total, target, Operator::Concat);
}

fn part_two(input: &String) {
    let total_calibration = input
		.split("\n")
		.map(|line| {
			let mut parts = line.trim().split(":");

			let target = parse_num(parts.next().unwrap());
			let nums: Vec<usize> = parts.next().unwrap().trim().split(" ").map(parse_num).collect();

			(target, nums)
		})
		.filter(|(target, nums)| view_node(0, &nums, 0, target, Operator::Add))
		.fold(0, |acc, (target, _nums)| acc + target);

	println!("Total calibration with concat operator: {}", total_calibration);
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    part_one(&input);
	part_two(&input);

    Ok(())
}
