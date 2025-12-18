use anyhow::Result;
use std::{fs, path::Path};

pub fn task_01(data_path: &Path) -> Result<String> {
    let content = fs::read_to_string(data_path)?;
    let splits = content.split("\n\n").collect::<Vec<_>>();
    let load_present_area = |present: &str| {
        present
            .chars()
            .map(|ch| match ch {
                '#' => 1,
                _ => 0,
            })
            .sum::<usize>()
    };
    let presents_area = splits
        .iter()
        .take(splits.len() - 1)
        .map(|present| load_present_area(present))
        .collect::<Vec<usize>>();
    let mut total = 0;
    for line in splits[splits.len() - 1]
        .split('\n')
        .filter(|line| !line.is_empty())
    {
        println!("line: {}", line);
        let line_split = line.split(':').collect::<Vec<_>>();
        let total_area = line_split[0]
            .split('x')
            .map(|ch| ch.parse::<usize>().unwrap())
            .product::<usize>();
        let present_area = line_split[1]
            .split_whitespace()
            .filter(|ch| !ch.is_empty())
            .map(|present| present.parse::<usize>().unwrap())
            .enumerate()
            .map(|(present_id, present_count)| presents_area[present_id] * present_count)
            .sum::<usize>();
        println!("present_area: {}, total_area: {}", present_area, total_area);
        total += if present_area <= total_area { 1 } else { 0 };
    }
    Ok(format!("Answer: {}", total))
}
