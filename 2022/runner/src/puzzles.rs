use day01;

pub fn puzzles() -> &'static [(&'static str, fn() -> usize, fn() -> usize)] {
    &[("day01", day01::day01a, day01::day01b)]
}
