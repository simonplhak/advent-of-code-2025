mod day_01;
mod day_02;
mod day_03;
mod utils;
use std::{fmt::Display, path::PathBuf};

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "cli_app", version = "1.0", about = "", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    second: bool,

    #[arg(short, long)]
    test: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Day01,
    Day02,
    Day03,
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commands::Day01 => write!(f, "day_01"),
            Commands::Day02 => write!(f, "day_02"),
            Commands::Day03 => write!(f, "day_03"),
        }
    }
}

fn data_path(cli: &Cli) -> PathBuf {
    PathBuf::from(format!(
        "data/{}/{}.txt",
        cli.command,
        match cli.test {
            true => "test",
            false => "input",
        }
    ))
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let res = match &cli.command {
        Commands::Day01 => match cli.second {
            true => day_01::task_02(&data_path(&cli))?,
            false => day_01::task_01(&data_path(&cli))?,
        },
        Commands::Day02 => match cli.second {
            true => day_02::task_02(&data_path(&cli))?,
            false => day_02::task_01(&data_path(&cli))?,
        },
        Commands::Day03 => match cli.second {
            true => day_03::task_02(&data_path(&cli))?,
            false => day_03::task_01(&data_path(&cli))?,
        },
    };
    println!("{:?}: {}", cli.command, res);
    Ok(())
}
