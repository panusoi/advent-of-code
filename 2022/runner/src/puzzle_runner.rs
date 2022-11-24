use crate::benchmark;
use crate::puzzles;
use crate::timing;

fn is_valid_puzzle_day(day_arg: &str) -> usize {
    let day_int = day_arg.parse::<usize>();

    if day_int.is_err() {
        eprintln!("Invalid day {}", day_arg);
        std::process::exit(1);
    }

    let day_int = day_int.unwrap();
    if day_int > puzzles::puzzles().len() - 1 {
        eprintln!("Invalid day {}", day_arg);
        std::process::exit(1);
    }

    return day_int;
}

fn run_puzzle(
    puzzle: &(&str, fn() -> usize, fn() -> usize),
) -> (timing::TimingResult, timing::TimingResult) {
    let day = puzzle.0;

    let a_fn = puzzle.1;
    let a_result = a_fn();
    let a_bench = benchmark::do_benchmark(a_fn);

    let b_fn = puzzle.2;
    let b_result = b_fn();
    let b_bench = benchmark::do_benchmark(b_fn);

    println!(
        "{}\n a\n  result: {}\n  benchmark: {}\n b\n  result: {}\n  benchmark:  {}",
        day, a_result, a_bench, b_result, b_bench
    );

    return (a_bench, b_bench);
}

pub fn run_puzzle_day(day_arg: &str) {
    let day = is_valid_puzzle_day(day_arg);
    run_puzzle(&puzzles::puzzles()[day]);
}

pub fn run_all_puzzles() {
    let benchmarks_times: Vec<_> = puzzles::puzzles().iter().map(|p| (run_puzzle(p))).collect();
    let total_benchmark_time = benchmarks_times
        .into_iter()
        .flat_map(|bt| [bt.0.get_guration(), bt.1.get_guration()])
        .sum();
    println!(
        "Total {}",
        timing::TimingResult::from_duration(total_benchmark_time)
    );
}
