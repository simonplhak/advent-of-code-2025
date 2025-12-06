use anyhow::Result;
use std::path::Path;

use crate::utils::read_lines;

pub fn task_01(data_path: &Path) -> Result<String> {
    let content = read_lines(data_path)?;
    let cells = content
        .iter()
        .take(content.len() - 1)
        .map(|line| {
            line.split(' ')
                .filter(|s| !s.is_empty())
                .map(|cell| cell.parse::<usize>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;
    let ops = content
        .last()
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let cols = cells[0].len();
    assert!(cells.iter().all(|row| row.len() == cols));
    assert!(ops.len() == cols);

    let mut grand_total = 0;
    for col in 0..cols {
        let op = ops[col];
        let mut col_total = match op {
            "+" => 0,
            "*" => 1,
            _ => panic!("Unknown op: {}", op),
        };
        for row in &cells {
            let val: usize = row[col];
            match op {
                "+" => col_total += val,
                "*" => col_total *= val,
                _ => panic!("Unknown op: {}", op),
            }
        }
        grand_total += col_total;
    }
    Ok(format!("Answer: {}", grand_total))
}

pub fn task_02(data_path: &Path) -> Result<String> {
    let content = read_lines(data_path)?;
    let mut ops = Vec::new();
    let mut ops_positions = Vec::new();
    for (i, c) in content.last().unwrap().chars().enumerate() {
        if c != ' ' {
            ops.push(c);
            ops_positions.push(i);
        }
    }
    let mut cells = vec![vec!["".to_string(); content.len() - 1]; ops_positions.len()];
    for line in content.iter().take(content.len() - 1) {
        for col_idx in 0..ops_positions.len() {
            let cell_str = line
                .get(ops_positions[col_idx]..*ops_positions.get(col_idx + 1).unwrap_or(&line.len()))
                .unwrap();
            for (cell_idx, c) in cell_str.chars().enumerate() {
                if c.is_whitespace() {
                    continue;
                }
                cells[col_idx][cell_idx] = format!("{}{}", cells[col_idx][cell_idx], c);
            }
        }
    }
    let cells = cells
        .into_iter()
        .zip(ops.iter())
        .map(|(col, op)| {
            col.into_iter()
                .map(|cell_str| {
                    cell_str.parse::<usize>().unwrap_or(match *op {
                        '+' => 0,
                        '*' => 1,
                        _ => panic!("Unknown op: {}", op),
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    assert!(ops.len() == cells.len());

    let mut grand_total = 0;
    for (col_idx, col) in cells.iter().enumerate() {
        let op = ops[col_idx];
        let mut col_total = match op {
            '+' => 0,
            '*' => 1,
            _ => panic!("Unknown op: {}", op),
        };
        for val in col {
            match op {
                '+' => col_total += *val,
                '*' => col_total *= *val,
                _ => panic!("Unknown op: {}", op),
            }
        }
        grand_total += col_total;
    }
    Ok(format!("Answer: {}", grand_total))
}
