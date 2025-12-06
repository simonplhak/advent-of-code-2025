use anyhow::Result;
use std::{collections::VecDeque, path::Path};

use crate::utils::{Range, RangedBinaryTree, create_range, read_lines};

pub fn task_01(data_path: &Path) -> Result<String> {
    let storage = load_storage(data_path)?;
    let tree = RangedBinaryTree::from(&storage.fresh);
    let total_available: usize = storage
        .available
        .iter()
        .filter(|value| tree.search(**value))
        .count();

    Ok(format!("Total available numbers: {}", total_available))
}

pub fn task_02(data_path: &Path) -> Result<String> {
    let storage = load_storage(data_path)?;
    let unique_ranges = storage.fresh.into_iter().fold(Vec::new(), add_unique_range);
    let total = unique_ranges
        .iter()
        .map(|r| r.end - r.start + 1)
        .sum::<usize>();
    Ok(format!("Total numbers in ranges: {}", total))
}

fn add_unique_range(unique_ranges: Vec<Range>, mut range: Range) -> Vec<Range> {
    let mut new = Vec::new();
    let mut deque = VecDeque::from(unique_ranges);
    while !deque.is_empty() {
        let current = deque.pop_back().unwrap();
        if current.overlaps(&range) {
            range = current.merge(&range);
        } else {
            new.push(current);
        }
    }
    new.push(range);
    new
}

#[derive(Debug)]
struct Storage {
    fresh: Vec<Range>,
    available: Vec<usize>,
}

fn load_storage(data_path: &Path) -> Result<Storage> {
    let lines = read_lines(data_path)?;
    let mut fresh = Vec::new();
    let mut available = Vec::new();

    let mut split_happened = false;
    for line in lines.iter() {
        match line.is_empty() {
            true => split_happened = true,
            false => match split_happened {
                true => available.push(line.parse::<usize>()?),
                false => fresh.push(create_range(line)?),
            },
        }
    }
    Ok(Storage { fresh, available })
}
