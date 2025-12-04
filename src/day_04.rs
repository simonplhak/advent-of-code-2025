use anyhow::Result;
use std::path::Path;

use crate::utils::read_lines;

pub fn task_01(data_path: &Path) -> Result<String> {
    let map = load_map(data_path)?;
    let valid_cells = find_valid_cells(&map);
    Ok(format!("Found {} valid cells.", valid_cells.len()))
}

fn find_valid_cells(map: &[Vec<Cell>]) -> Vec<(usize, usize)> {
    let mut cells = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == Cell::Filled {
                let adjacent = get_adjacent_cells(map, row, col);
                let filled_count = adjacent
                    .into_iter()
                    .filter(|(r, c)| map[*r][*c] == Cell::Filled)
                    .count();
                if filled_count < 4 {
                    cells.push((row, col));
                }
            }
        }
    }
    cells
}

pub fn task_02(data_path: &Path) -> Result<String> {
    let mut map = load_map(data_path)?;
    let mut removed_cells = 0;
    loop {
        let valid_cells = find_valid_cells(&map);
        if valid_cells.is_empty() {
            break;
        }
        for (row, col) in valid_cells.iter() {
            map[*row][*col] = Cell::Empty;
        }
        removed_cells += valid_cells.len();
    }

    Ok(format!("Removed {} cells.", removed_cells))
}

#[derive(PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    Filled,
}

fn load_map(data_path: &Path) -> Result<Vec<Vec<Cell>>> {
    let lines = read_lines(data_path)?;
    let map: Vec<Vec<Cell>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Cell::Empty,
                    '@' => Cell::Filled,
                    _ => panic!("Unexpected character in map"),
                })
                .collect()
        })
        .collect();
    Ok(map)
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn get_adjacent_cells(map: &[Vec<Cell>], row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut adjacent = Vec::new();
    for (dr, dc) in DIRECTIONS.iter() {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;
        if new_row >= 0
            && new_row < map.len() as isize
            && new_col >= 0
            && new_col < map[0].len() as isize
        {
            adjacent.push((new_row as usize, new_col as usize));
        }
    }
    assert!(adjacent.len() <= 8);
    adjacent
}
