use std::fs;

use anyhow::Result;

type Position = (usize, usize);

fn check_xmas(grid: &Vec<Vec<char>>, pos2: Position, pos3: Position, pos4: Position, count: &mut usize) {
    if grid[pos2.0][pos2.1] == 'M' && grid[pos3.0][pos3.1] == 'A' && grid[pos4.0][pos4.1] == 'S' {
        *count += 1;
    }
}

fn part_one(input: &String) {
    let grid: Vec<Vec<char>> = input.split("\n").map(|str| str.trim().chars().collect()).collect();

    let grid_x = grid.first().unwrap().len();
    let grid_y = grid.len();

    let mut count = 0;

    for x in 0..grid_x {
        for y in 0..grid_y {
            let current = grid[x][y];

            if current != 'X' { continue; }

            // right
            if x < grid_x - 3 {
                check_xmas(&grid, (x + 1, y), (x + 2, y), (x + 3, y), &mut count);

                // up right
                if y > 2 {
                    check_xmas(&grid, (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3), &mut count);
                }

                // down right
                if y < grid_y - 3 {
                    check_xmas(&grid, (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3), &mut count);
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
                    check_xmas(&grid, (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3), &mut count);
                }

                // down left
                if y < grid_y - 3 {
                    check_xmas(&grid, (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3), &mut count);
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

fn check_cross_mas(grid: &Vec<Vec<char>>) {

}

fn part_two(input: &String) {
    let grid: Vec<Vec<char>> = input.split("\n").map(|str| str.trim().chars().collect()).collect();


}

fn main() -> Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");

    let input = fs::read_to_string("input.txt")?;

    part_one(&input);

    Ok(())
}
