use std::path::Path;
use std::str::FromStr;

use crate::utils::read_lines;
use anyhow::{Result, anyhow};

pub fn task_01(data_path: &Path) -> Result<String> {
    let lines = read_lines(data_path)?;
    let instructions = lines
        .into_iter()
        .map(|line| line.trim().parse())
        .collect::<Result<Vec<Instruction>>>()?;
    let mut curr = 50; // starting point
    let mut zeros_hit = 0;
    for instruction in &instructions {
        let value = instruction.value % 100;
        curr = match instruction.rot {
            Rotation::L => match curr < value {
                true => 100 - ((100 + value - curr) % 100),
                false => curr - value,
            },
            Rotation::R => (curr + value) % 100,
        };
        assert!(curr < 100);
        if curr == 0 {
            zeros_hit += 1;
        }
    }
    Ok(format!(
        "Final position: {}, Zeros hit: {}",
        curr, zeros_hit
    ))
}

pub fn task_02(data_path: &Path) -> Result<String> {
    let lines = read_lines(data_path)?;
    let instructions = lines
        .into_iter()
        .map(|line| line.trim().parse())
        .collect::<Result<Vec<Instruction>>>()?;
    let mut curr = 50; // starting point
    let mut zeros_hit = 0;
    for instruction in &instructions {
        // println!("{}", instruction.value / 100);
        zeros_hit += instruction.value / 100;
        let value = instruction.value % 100;
        let new = match instruction.rot {
            Rotation::L => match curr < value {
                true => {
                    if curr != 0 {
                        zeros_hit += 1;
                    }
                    100 - ((100 + value - curr) % 100)
                }
                false => curr - value,
            },
            Rotation::R => curr + value,
        };
        println!("{}", new / 100);
        if new != 100 {
            zeros_hit += new / 100;
        }
        curr = new % 100;
        assert!(curr < 100);
        if curr == 0 {
            zeros_hit += 1;
        }
        println!("After {:?}: pos {}, zeros {}", instruction, curr, zeros_hit);
    }
    Ok(format!(
        "Final position: {}, Zeros hit: {}",
        curr, zeros_hit
    ))
}

#[derive(Debug)]
pub enum Rotation {
    L,
    R,
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "L" => Ok(Rotation::L),
            "R" => Ok(Rotation::R),
            _ => Err(anyhow!("invalid rotation: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub rot: Rotation,
    pub value: u32,
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(anyhow!("instruction too short: {}", s));
        }
        let rot_part = &s[0..1];
        let num_part = &s[1..];
        let rot = rot_part.parse()?;
        let value = num_part
            .parse::<u32>()
            .map_err(|e| anyhow!("invalid number: {}", e))?;
        Ok(Instruction { rot, value })
    }
}
