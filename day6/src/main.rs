use std::{cell::RefCell, fs, rc::Rc, time::SystemTime};

use anyhow::Result;

enum TileType {
    Empty,
    Obstacle,
}

type TileReference = Rc<RefCell<Tile>>;
type MapColumn = Vec<TileReference>;
type Map = Vec<MapColumn>;

struct Tile {
    x: usize,
    y: usize,
    tile_type: TileType,
    last_visited_via: Option<MoveDirection>,
}

impl Tile {
    pub fn new(character: char, x: usize, y: usize) -> Self {
        Self {
            x,
            y,

            tile_type: match character {
                '#' => TileType::Obstacle,
                '.' | _ => TileType::Empty,
            },

            last_visited_via: None,
        }
    }

    pub fn reset(&mut self) {
        self.last_visited_via = None;
    }
}

#[derive(Clone, Eq, PartialEq)]
enum MoveDirection {
    Up,
    Left,
    Down,
    Right,
}

struct Guard {
    original_tile: TileReference,
    current_tile: TileReference,

    moves: usize,
    unique_moves: usize,
    did_move_onto_tile_with_same_dir: bool,
    move_direction: MoveDirection,
}

impl Guard {
    pub fn new(start_tile: &TileReference) -> Self {
        Self {
            original_tile: start_tile.clone(),
            current_tile: start_tile.clone(),
            moves: 1,
            unique_moves: 1,
            did_move_onto_tile_with_same_dir: false,
            move_direction: MoveDirection::Up,
        }
    }

    pub fn move_to_next_tile(&mut self, map: &Map, map_x: usize, map_y: usize) -> bool {
        let current_x = self.current_tile.borrow().x as isize;
        let current_y = self.current_tile.borrow().y as isize;

        let (next_x, next_y) = match self.move_direction {
            MoveDirection::Up => (current_x - 1, current_y),
            MoveDirection::Left => (current_x, current_y - 1),
            MoveDirection::Down => (current_x + 1, current_y),
            MoveDirection::Right => (current_x, current_y + 1),
        };

        if next_x < 0
            || next_x > (map_x - 1) as isize
            || next_y < 0
            || next_y > (map_y - 1) as isize
        {
            return false;
        }

        let next_tile = map[next_x as usize][next_y as usize].clone();

        if let TileType::Obstacle = next_tile.borrow().tile_type {
            self.move_direction = match self.move_direction {
                MoveDirection::Up => MoveDirection::Right,
                MoveDirection::Left => MoveDirection::Up,
                MoveDirection::Down => MoveDirection::Left,
                MoveDirection::Right => MoveDirection::Down,
            };

            return self.move_to_next_tile(map, map_x, map_y);
        }

        if let Some(last_direction) = next_tile.borrow().last_visited_via.as_ref() {
            self.did_move_onto_tile_with_same_dir = *last_direction == self.move_direction;
        } else {
            self.unique_moves += 1;
        }

        next_tile.borrow_mut().last_visited_via = Some(self.move_direction.clone());

        self.current_tile = next_tile;
        self.moves += 1;

        return true;
    }

    fn reset(&mut self) {
        self.current_tile = self.original_tile.clone();
        self.move_direction = MoveDirection::Up;
        self.moves = 1;
        self.unique_moves = 1;
        self.did_move_onto_tile_with_same_dir = false;
    }
}

fn part_one(input: &String) {
    let char_map: Vec<Vec<char>> = input
        .trim()
        .split("\n")
        .map(|line| line.trim().chars().collect())
        .collect();

    let map_x = char_map.first().unwrap().len();
    let map_y = char_map.len();

    let mut map: Map = Vec::with_capacity(map_x);
    let mut guard: Option<Guard> = None;

    for x in 0..map_x {
        let mut map_column_vec: MapColumn = Vec::with_capacity(map_x);

        for y in 0..map_y {
            let character = char_map[x][y];

            if character == '^' {
                let start_tile = Rc::new(RefCell::new(Tile::new('.', x, y)));
                guard = Some(Guard::new(&start_tile));

                map_column_vec.push(start_tile);

                continue;
            }

            map_column_vec.push(Rc::new(RefCell::new(Tile::new(character, x, y))));
        }

        map.push(map_column_vec);
    }

    let mut guard = guard.expect("did not find guard");

    while guard.move_to_next_tile(&map, map_x, map_y) {}

    println!("Unique moves: {}", guard.unique_moves);
}

fn part_two(input: &String) {
    let char_map: Vec<Vec<char>> = input
        .trim()
        .split("\n")
        .map(|line| line.trim().chars().collect())
        .collect();

    let map_x = char_map.first().unwrap().len();
    let map_y = char_map.len();

    let mut map: Map = Vec::with_capacity(map_x);
    let mut guard: Option<Guard> = None;

    for x in 0..map_x {
        let mut map_column_vec: MapColumn = Vec::with_capacity(map_x);

        for y in 0..map_y {
            let character = char_map[x][y];

            if character == '^' {
                let start_tile = Rc::new(RefCell::new(Tile::new('.', x, y)));
                guard = Some(Guard::new(&start_tile));

                map_column_vec.push(start_tile);

                continue;
            }

            map_column_vec.push(Rc::new(RefCell::new(Tile::new(character, x, y))));
        }

        map.push(map_column_vec);
    }

    let mut guard = guard.expect("did not find guard");
    let original_x = guard.original_tile.borrow().x;
    let original_y = guard.original_tile.borrow().y;

    let mut obstructed_count = 0;

    for x in 0..map_x {
        for y in 0..map_y {
            if x == original_x && y == original_y {
                continue;
            }

            let mut new_obstacle_tile = map[x][y].borrow_mut();

            if let TileType::Obstacle = new_obstacle_tile.tile_type {
                continue;
            }

            new_obstacle_tile.tile_type = TileType::Obstacle;
            drop(new_obstacle_tile);

            while guard.move_to_next_tile(&map, map_x, map_y) {
                if guard.did_move_onto_tile_with_same_dir {
                    obstructed_count += 1;
                    break;
                }
            }

            map[x][y].borrow_mut().tile_type = TileType::Empty;

            for x in 0..map_x {
                for y in 0..map_y {
                    map[x][y].borrow_mut().reset();
                }
            }

            guard.reset();
        }
    }

    println!("Potential obstacles to cause a loop: {}", obstructed_count);
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    part_one(&input);

    let start = SystemTime::now();

    part_two(&input);

    let total = SystemTime::now().duration_since(start)?;
    println!("Time for pt. 2: {}s", total.as_secs_f32());

    Ok(())
}
