use day01;
use day02;

pub fn puzzles() -> &'static [(&'static str, fn() -> usize, fn() -> usize)] {
    &[
        ("day01", day01::day01a, day01::day01b),
        ("day02", day02::day02a, day02::day02b),
    ]
}
