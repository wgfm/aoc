mod gen;
mod problem;
mod site;

mod y2015;
mod y2020;
mod y2023;
mod y2024;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]

struct Aoc {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Run {
        #[arg(short, long)]
        year: u64,

        #[arg(short, long)]
        day: usize,

        #[arg(short, long)]
        part: char,
    },
    Gen {
        #[arg(short, long)]
        year: u64,
    },
}

fn main() {
    let args = Aoc::parse();
    match args.command {
        Commands::Run { year, day, part } => run(year, day, part),
        Commands::Gen { year } => gen(year),
    }
}

fn run(year: u64, day: usize, part: char) {
    let problems = problem::problems_for_year(year).expect("cannot fetch input");
    if let Some(problem) = problems.get(day - 1) {
        let solution = match part {
            'A' | 'a' => problem.part_a(),
            'B' | 'b' => problem.part_b(),
            _ => Err(anyhow::anyhow!("no such part: {}", part)),
        }
        .expect("Could not solve problem");

        println!("{}", solution);
    } else {
        eprintln!(
            "no solution found for year {}, day {}, part {}",
            year, day, part
        );
        return;
    }
}

fn gen(y: u64) {
    if let Err(e) = gen::year(y) {
        eprintln!("{}", e);
    }
}
