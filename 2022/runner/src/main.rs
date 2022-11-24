use std::env;

mod benchmark;
mod puzzle_runner;
mod puzzles;
mod timing;

fn print_title() {
    let advent_of_code = "Advent of Code 2022";
    println!(
        "{}\n{}",
        advent_of_code,
        std::iter::repeat("*")
            .take(advent_of_code.len())
            .collect::<String>()
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!(
            "Running benchmark for all puzzles. Run count is {}.\n",
            benchmark::RUNS
        );
        print_title();
        puzzle_runner::run_all_puzzles();
        std::process::exit(0);
    }

    let day = &args[1];
    puzzle_runner::run_puzzle_day(day);
}
