mod problem;
mod site;

mod y2015;
mod y2020;
mod y2023;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    year: u64,

    #[arg(short, long)]
    day: usize,

    #[arg(short, long)]
    part: char,
}

fn main() {
    let args = Args::parse();

    let problems = problem::problems_for_year(args.year).expect("cannot fetch input");
    if let Some(problem) = problems.get(args.day - 1) {
        let solution = match args.part {
            'A' | 'a' => problem.part_a(),
            'B' | 'b' => problem.part_b(),
            _ => Err(anyhow::anyhow!("no such part: {}", args.part)),
        }
        .expect("Could not solve problem");

        println!("{}", solution);
    } else {
        eprintln!(
            "no solution found for year {}, day {}, part {}",
            args.year, args.day, args.part
        );
        return;
    }
}
