use anyhow::Result;
use good_lp::{
    Expression, ProblemVariables, Solution as _, SolverModel, constraint, default_solver, variable,
};
use std::{collections::HashMap, path::Path};

use crate::utils::read_lines;

pub fn task_01(data_path: &Path) -> Result<String> {
    let instructions = load_instructions(data_path)?;
    let mut total_switches = 0;
    for instruction in &instructions {
        let mut curr_state = Grid(vec![false; instruction.grid.0.len()]);
        let mut visited_states = HashMap::new();
        let min_depth =
            resolve_light_instructions(instruction, &mut curr_state, 0, &mut visited_states)
                .unwrap();
        total_switches += min_depth;
    }
    Ok(format!("Answer: {}", total_switches))
}

pub fn task_02(data_path: &Path) -> Result<String> {
    let instructions = load_instructions(data_path)?;
    let mut total_switches = 0;
    for (i, instruction) in instructions.iter().enumerate() {
        println!("{i}/{:?}", instructions.len());
        let min_depth = resolve_joltage_instructions(instruction)?;
        total_switches += min_depth;
    }
    Ok(format!("Answer: {}", total_switches))
}

fn resolve_light_instructions(
    instruction: &Instruction,
    curr_state: &mut Grid,
    depth: usize,
    visited_states: &mut HashMap<usize, usize>,
) -> Option<usize> {
    let mut min_switches = None;
    for button in 0..instruction.buttons.len() {
        let orig_curr_state_num = curr_state.to_num();
        instruction.press_button(button, curr_state);
        let curr_state_num = curr_state.to_num();
        if instruction.grid_num == curr_state_num {
            instruction.press_button(button, curr_state);
            return Some(depth + 1);
        }
        let mut to_continue = true;
        visited_states
            .entry(curr_state_num)
            .and_modify(|entry_depth| {
                if depth < *entry_depth {
                    *entry_depth = depth;
                } else {
                    to_continue = false;
                }
            })
            .or_insert(depth);
        if to_continue {
            let res =
                resolve_light_instructions(instruction, curr_state, depth + 1, visited_states);
            match (res, min_switches) {
                (Some(r), Some(m)) => {
                    if r < m {
                        min_switches = Some(r);
                    }
                }
                (Some(r), None) => {
                    min_switches = Some(r);
                }
                _ => {}
            }
        }
        // rollback operation
        instruction.press_button(button, curr_state);
        assert!(curr_state.to_num() == orig_curr_state_num);
    }
    min_switches
}

fn resolve_joltage_instructions(instruction: &Instruction) -> Result<usize> {
    let mut vars = ProblemVariables::new();
    let x = (0..instruction.buttons.len())
        .map(|_| vars.add(variable().min(0).integer()))
        .collect::<Vec<_>>();
    let objective = x.iter().fold(Expression::from(0), |acc, v| acc + v);
    let mut constraints = Vec::new();
    for (target_idx, target) in instruction.joltage.0.iter().enumerate() {
        let affecting_buttons = instruction
            .buttons
            .iter()
            .enumerate()
            .filter(|(_, button)| button.switches.contains(&target_idx))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        let expr = affecting_buttons
            .iter()
            .fold(Expression::from(0), |acc, &i| acc + x[i]);
        let constraint = constraint!(Expression::from(*target as i32) == expr);
        constraints.push(constraint);
    }

    let solution = vars
        .minimise(objective)
        .using(default_solver)
        .with_all(constraints)
        .solve()?;
    Ok((0..instruction.buttons.len())
        .map(|i| solution.value(x[i]) as usize)
        .sum())
}

#[derive(Debug)]
struct Grid(Vec<bool>);

impl Grid {
    fn to_num(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .fold(0usize, |acc, (i, &b)| acc + if b { 1 << i } else { 0 })
    }
}

#[derive(Debug)]
struct Button {
    switches: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Joltage(Vec<usize>);

#[derive(Debug)]
struct Instruction {
    grid: Grid,
    grid_num: usize,
    buttons: Vec<Button>,
    joltage: Joltage,
}

impl Instruction {
    fn press_button(&self, index: usize, curr_state: &mut Grid) {
        let button = &self.buttons[index];
        for &switch in &button.switches {
            curr_state.0[switch] = !curr_state.0[switch];
        }
    }
}

fn load_instructions(data_path: &Path) -> Result<Vec<Instruction>> {
    let lines = read_lines(data_path)?;
    let instructions = lines
        .iter()
        .map(|line| {
            // parse line into Instruction
            let splits = line.split(' ').collect::<Vec<_>>();
            let grid = splits[0];
            let buttons = &splits[1..splits.len() - 1];
            let joltage = splits[splits.len() - 1];
            let grid = Grid(
                grid[1..grid.len() - 1]
                    .chars()
                    .map(|c| c == '#')
                    .collect::<Vec<bool>>(),
            );
            let joltage = Joltage(
                joltage[1..joltage.len() - 1]
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            );
            let grid_num = grid.to_num();
            Ok(Instruction {
                grid,
                grid_num,
                buttons: buttons
                    .iter()
                    .map(|b| {
                        let switch_str = &b[1..b.len() - 1];
                        let switches = switch_str
                            .split(',')
                            .map(|s| s.trim().parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();
                        Button { switches }
                    })
                    .collect::<Vec<Button>>(),
                joltage,
            })
        })
        .collect::<Result<Vec<Instruction>>>()?;
    Ok(instructions)
}
