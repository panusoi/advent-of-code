use day00;

pub fn puzzles() -> &'static [(&'static str, fn() -> usize, fn() -> usize)] {
    &[("day00", day00::day00a, day00::day00b)]
}
