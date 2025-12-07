use anyhow::Result;
use core::panic;
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

use crate::utils::read_lines;

pub fn task_01(data_path: &Path) -> Result<String> {
    let map = load_map(data_path)?;
    assert!(map.start.0 == 0);
    let mut beans = HashSet::new();
    let mut splits = 0;
    beans.insert(map.start.1);
    for (row_idx, row) in map.cells.iter().skip(1).enumerate() {
        let mut new_beans = HashSet::new();
        for bean in &beans {
            match map.cells[row_idx + 1][*bean] {
                Cell::Empty => {
                    let _ = new_beans.insert(*bean);
                }
                Cell::Split => {
                    splits += 1;
                    if *bean > 0 {
                        new_beans.insert(*bean - 1);
                    }
                    if *bean < row.len() - 1 {
                        new_beans.insert(*bean + 1);
                    }
                }
                Cell::Start => panic!(),
            };
        }
        beans = new_beans;
    }
    Ok(format!("Beam splitted: {}", splits))
}

pub fn task_02(data_path: &Path) -> Result<String> {
    let map = load_map(data_path)?;
    assert!(map.start.0 == 0);
    let mut beans = HashMap::new();
    beans.insert(map.start.1, 1);
    for (row_idx, row) in map.cells.iter().skip(1).enumerate() {
        let mut new_beans = HashMap::new();
        for (bean, count) in &beans {
            match map.cells[row_idx + 1][*bean] {
                Cell::Empty => {
                    let _ = new_beans
                        .entry(*bean)
                        .and_modify(|counter| *counter += *count)
                        .or_insert(*count);
                }
                Cell::Split => {
                    if *bean > 0 {
                        new_beans
                            .entry(*bean - 1)
                            .and_modify(|counter| *counter += *count)
                            .or_insert(*count);
                    }
                    if *bean < row.len() - 1 {
                        new_beans
                            .entry(*bean + 1)
                            .and_modify(|counter| *counter += *count)
                            .or_insert(*count);
                    }
                }
                Cell::Start => panic!(),
            };
        }
        beans = new_beans;
    }
    let total_beans: usize = beans.values().sum();
    Ok(format!("Beam splitted: {}", total_beans))
}

#[derive(Debug)]
enum Cell {
    Empty,
    Split,
    Start,
}

struct Map {
    cells: Vec<Vec<Cell>>,
    start: (usize, usize),
}

fn load_map(data_path: &Path) -> Result<Map> {
    let lines = read_lines(data_path)?;
    let mut start = None;
    let map = lines
        .iter()
        .enumerate()
        .map(|(pos, line)| {
            line.chars()
                .enumerate()
                .map(|(ch_pos, ch)| match ch {
                    '.' => Cell::Empty,
                    '^' => Cell::Split,
                    'S' => {
                        start = Some((pos, ch_pos));
                        Cell::Start
                    }
                    _ => Cell::Empty,
                })
                .collect()
        })
        .collect();
    Ok(Map {
        cells: map,
        start: start.unwrap(),
    })
}
