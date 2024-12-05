use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use anyhow::Result;

fn parse_num(str: &str) -> usize {
    return str.parse().unwrap();
}

fn part_one(input: &String) {
    let parts: Vec<&str> = input.split("ORDERING_RULES_END").collect();

    let ordering_rules = parts[0].trim();
    let updates = parts[1].trim();

    let rules: HashMap<usize, HashSet<usize>> = ordering_rules
        .split("\n")
        .map(|rule| {
            rule.trim()
                .split("|")
                .map(parse_num)
                .collect::<Vec<usize>>()
        })
        .fold(HashMap::new(), |mut hashmap, nums: Vec<usize>| {
            hashmap.entry(nums[0]).or_default().insert(nums[1]);

            hashmap
        });

    let correct_updates: Vec<Vec<usize>> = updates
        .split("\n")
        .map(|update| update.trim().split(",").map(parse_num).collect())
        .filter(|update: &Vec<usize>| {
            (0..update.len() - 1).all(|i| rules.get(&update[i]).unwrap().contains(&update[i + 1]))
        })
        .collect();

    let middle_sum = correct_updates.into_iter().fold(0, |acc, update| {
        acc + update[(update.len() - 1).div_ceil(2)]
    });

    println!("Sums of middles: {}", middle_sum);
}

fn part_two(input: &String) {
    let parts: Vec<&str> = input.split("ORDERING_RULES_END").collect();

    let ordering_rules = parts[0].trim();
    let updates = parts[1].trim();

    let rules: HashMap<usize, HashSet<usize>> = ordering_rules
        .split("\n")
        .map(|rule| {
            rule.trim()
                .split("|")
                .map(parse_num)
                .collect::<Vec<usize>>()
        })
        .fold(HashMap::new(), |mut hashmap, nums: Vec<usize>| {
            hashmap.entry(nums[0]).or_default().insert(nums[1]);

            hashmap
        });

    let update_sorter = |a: &usize, b: &usize| {
        if rules.get(a).unwrap().contains(b) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    };

    let sorted_incorrect_updates: Vec<Vec<usize>> = updates
        .split("\n")
        .map(|update| update.trim().split(",").map(parse_num).collect())
        .filter(|update: &Vec<usize>| {
            (0..update.len() - 1).any(|i| !rules.get(&update[i]).unwrap().contains(&update[i + 1]))
        })
        .map(|mut update| {
            update.sort_by(update_sorter);
            update
        })
        .collect();

    let middle_sum = sorted_incorrect_updates.into_iter().fold(0, |acc, update| {
        acc + update[(update.len() - 1).div_ceil(2)]
    });

    println!("Sums of incorrect middles: {}", middle_sum);
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    part_one(&input);
    part_two(&input);

    Ok(())
}
