use std::fs;

use anyhow::Result;

type Position = (usize, usize);
type Grid = Vec<Vec<char>>;

fn at_pos_is(grid: &Grid, position: Position, character: char) -> bool {
    return grid[position.0][position.1] == character;
}

fn check_xmas(grid: &Grid, pos1: Position, pos2: Position, pos3: Position, count: &mut usize) {
    if at_pos_is(grid, pos1, 'M') && at_pos_is(grid, pos2, 'A') && at_pos_is(grid, pos3, 'S') {
        *count += 1;
    }
}

fn part_one(input: &String) {
    let grid: Grid = input
        .split("\n")
        .map(|str| str.trim().chars().collect())
        .collect();

    let grid_x = grid.first().unwrap().len();
    let grid_y = grid.len();

    let mut count = 0;

    for x in 0..grid_x {
        for y in 0..grid_y {
            let current = grid[x][y];

            if current != 'X' {
                continue;
            }

            // right
            if x < grid_x - 3 {
                check_xmas(&grid, (x + 1, y), (x + 2, y), (x + 3, y), &mut count);

                // up right
                if y > 2 {
                    check_xmas(
                        &grid,
                        (x + 1, y - 1),
                        (x + 2, y - 2),
                        (x + 3, y - 3),
                        &mut count,
                    );
                }

                // down right
                if y < grid_y - 3 {
                    check_xmas(
                        &grid,
                        (x + 1, y + 1),
                        (x + 2, y + 2),
                        (x + 3, y + 3),
                        &mut count,
                    );
                }
            }

            // up
            if y > 2 {
                check_xmas(&grid, (x, y - 1), (x, y - 2), (x, y - 3), &mut count);
            }

            // left
            if x > 2 {
                check_xmas(&grid, (x - 1, y), (x - 2, y), (x - 3, y), &mut count);

                // up left
                if y > 2 {
                    check_xmas(
                        &grid,
                        (x - 1, y - 1),
                        (x - 2, y - 2),
                        (x - 3, y - 3),
                        &mut count,
                    );
                }

                // down left
                if y < grid_y - 3 {
                    check_xmas(
                        &grid,
                        (x - 1, y + 1),
                        (x - 2, y + 2),
                        (x - 3, y + 3),
                        &mut count,
                    );
                }
            }

            // down
            if y < grid_y - 3 {
                check_xmas(&grid, (x, y + 1), (x, y + 2), (x, y + 3), &mut count);
            }
        }
    }

    println!("XMAS found: {}", count);
}

fn check_cross_mas(
    grid: &Grid,
    top_left: Position,
    top_right: Position,
    bottom_left: Position,
    bottom_right: Position,
    count: &mut usize,
) {
    let top_left_char = grid[top_left.0][top_left.1];
    let top_right_char = grid[top_right.0][top_right.1];

    let bottom_left_char = grid[bottom_left.0][bottom_left.1];
    let bottom_right_char = grid[bottom_right.0][bottom_right.1];

    let possibilities = vec![
        // S . S
        // . A .
        // M . M
        top_left_char == 'S'
            && top_right_char == 'S'
            && bottom_left_char == 'M'
            && bottom_right_char == 'M',
        // M . M
        // . A .
        // S . S
        top_left_char == 'M'
            && top_right_char == 'M'
            && bottom_left_char == 'S'
            && bottom_right_char == 'S',
        // M . S
        // . A .
        // M . S
        top_left_char == 'M'
            && top_right_char == 'S'
            && bottom_left_char == 'M'
            && bottom_right_char == 'S',
        // S . M
        // . A .
        // S . M
        top_left_char == 'S'
            && top_right_char == 'M'
            && bottom_left_char == 'S'
            && bottom_right_char == 'M',
    ];

    if possibilities.into_iter().any(|bool| bool) {
        *count += 1;
    }
}

fn part_two(input: &String) {
    let grid: Grid = input
        .split("\n")
        .map(|str| str.trim().chars().collect())
        .collect();

    let grid_x = grid.first().unwrap().len();
    let grid_y = grid.len();

    let mut count = 0;

    for x in 1..grid_x - 1 {
        for y in 1..grid_y - 1 {
            let current = grid[x][y];

            if current != 'A' {
                continue;
            }

            check_cross_mas(
                &grid,
                (x - 1, y - 1),
                (x + 1, y - 1),
                (x - 1, y + 1),
                (x + 1, y + 1),
                &mut count,
            );
        }
    }

    println!("X-MAS found: {}", count);
}

fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "0");

    let input = fs::read_to_string("input.txt")?;

    part_one(&input);
    part_two(&input);

    Ok(())
}
