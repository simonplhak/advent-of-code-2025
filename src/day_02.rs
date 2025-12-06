use std::path::Path;

use anyhow::Result;

use crate::utils::{Range, digit_count, make_ranges};

pub fn task_01(data_path: &Path) -> Result<String> {
    let ranges = load_data(data_path)?;
    let mut palindromic = Vec::new();
    for range in ranges {
        for i in range.start..=range.end {
            let digs = digit_count(i);
            if digs % 2 == 1 {
                continue;
            }
            let exp = 10usize.pow((digs / 2) as u32);
            let left = i / exp;
            let right = i % exp;
            if left == right {
                palindromic.push(i);
            }
        }
    }
    Ok(format!(
        "Total palindromic numbers: {}, Sum of palindromic numbers: {}",
        palindromic.len(),
        palindromic.iter().sum::<usize>()
    ))
}

pub fn task_02(data_path: &Path) -> Result<String> {
    let ranges = load_data(data_path)?;
    let mut palindromic = Vec::new();
    for range in ranges {
        for i in range.start..=range.end {
            let digs = digit_count(i);
            for splits in 1..=(digs / 2) {
                if digs.is_multiple_of(splits) {
                    continue;
                }
                let exps = (splits..digs)
                    .step_by(splits)
                    .map(|j| 10usize.pow(j as u32))
                    .collect::<Vec<usize>>();
                let base = 10usize.pow(splits as u32);
                let nums = exps
                    .iter()
                    .map(|exp| (i / exp) % base)
                    .chain(std::iter::once(i % base))
                    .collect::<Vec<usize>>();
                if nums.iter().all(|&num| num == nums[0]) {
                    println!(
                        "Found palindromic number: {} with splits {}, nums: {:?}",
                        i, splits, nums
                    );
                    palindromic.push(i);
                    break;
                }
            }
        }
    }
    Ok(format!(
        "Total palindromic numbers: {}, Sum of palindromic numbers: {}",
        palindromic.len(),
        palindromic.iter().sum::<usize>()
    ))
}

fn load_data(data_path: &Path) -> Result<Vec<Range>> {
    let content = std::fs::read_to_string(data_path)?
        .trim()
        .replace("\n", "")
        .to_string();
    make_ranges(content, ',')
}
