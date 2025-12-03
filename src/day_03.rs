use anyhow::Result;
use std::path::Path;

use crate::utils::read_lines;

pub fn task_01(data_path: &Path) -> Result<String> {
    let lines = read_lines(data_path)?;
    let mut biggest_nums = Vec::new();
    for line in lines {
        let mut biggest = 0;
        let mut second_biggest = None;
        for i in 0..line.len() {
            let dig = line.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
            if dig > biggest && i != line.len() - 1 {
                second_biggest = None;
                biggest = dig;
            } else if second_biggest.is_none() || dig > second_biggest.unwrap() {
                second_biggest = Some(dig);
            }
        }
        assert!(second_biggest.is_some());
        let biggest_num = biggest * 10 + second_biggest.unwrap();
        biggest_nums.push(biggest_num);
    }
    Ok(format!(
        "Sum of found biggest 2-digit numbers: {}",
        biggest_nums.iter().sum::<usize>(),
    ))
}

const DIGIT_COUNT: usize = 12;

pub fn task_02(data_path: &Path) -> Result<String> {
    let lines = read_lines(data_path)?;
    let mut biggest_nums = Vec::new();
    for line in lines {
        let mut biggest = 0;
        let mut next_biggest = vec![None; DIGIT_COUNT - 1];
        for i in 0..line.len() {
            let dig = line.chars().nth(i).unwrap().to_digit(10).unwrap() as usize;
            if dig > biggest && i <= line.len() - DIGIT_COUNT {
                next_biggest = vec![None; DIGIT_COUNT - 1];
                biggest = dig;
            } else {
                let start = match DIGIT_COUNT - 1 > line.len() - i {
                    true => DIGIT_COUNT - (line.len() - i) - 1,
                    false => 0,
                };
                let tmp = start..DIGIT_COUNT - 1;
                for j in tmp {
                    match &next_biggest[j] {
                        Some(num) => {
                            if dig > *num && DIGIT_COUNT - j < line.len() {
                                next_biggest[j] = Some(dig);
                                next_biggest[j + 1..].iter_mut().for_each(|n| *n = None);
                                break;
                            }
                        }
                        None => {
                            next_biggest[j] = Some(dig);
                            next_biggest[j + 1..].iter_mut().for_each(|n| *n = None);
                            break;
                        }
                    }
                }
            }
        }
        // Verification
        assert!(next_biggest.iter().all(|n| n.is_some()));
        let mut biggest_matched = false;
        let mut next_matched = [false; DIGIT_COUNT - 1];
        let mut pos = 0;
        for ch in line.chars() {
            let dig = ch.to_digit(10).unwrap() as usize;
            if dig == biggest && !biggest_matched {
                biggest_matched = true;
            } else if biggest_matched && dig == next_biggest[pos].unwrap() {
                next_matched[pos] = true;
                pos += 1;
                if pos >= DIGIT_COUNT - 1 {
                    break;
                }
            }
        }
        assert!(
            biggest_matched,
            "Biggest digit {} not matched for line: {}",
            biggest, line
        );
        assert!(
            next_matched.iter().all(|&m| m),
            "Not all next biggest digits matched for line: {}, next_biggest: {:?} found matches: {:?}",
            line,
            next_biggest,
            next_matched
        );
        let biggest_num = biggest * 10usize.pow((DIGIT_COUNT - 1) as u32)
            + next_biggest
                .iter()
                .map(|n| n.unwrap())
                .fold(0, |acc, n| acc * 10 + n);
        biggest_nums.push(biggest_num);
    }
    Ok(format!(
        "Sum of found biggest {}-digit numbers: {}",
        DIGIT_COUNT,
        biggest_nums.iter().sum::<usize>(),
    ))
}
