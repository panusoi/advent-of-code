use day01;
use day02;
use day03;
use day04;
use day05;
use day06;
use day07;
// use day08;
// use day09;
// use day10;
// use day11;
// use day13;
// use day14;
// use day15;
// use day16;
// use day17;
// use day18;
// use day18;
// use day19;
// use day20;
// use day21;
// use day22;
// use day23;
// use day24;
// use day25;

pub fn puzzles() -> &'static [(&'static str, fn() -> String, fn() -> String)] {
    &[
        ("day01", day01::day01a, day01::day01b),
        ("day02", day02::day02a, day02::day02b),
        ("day03", day03::day03a, day03::day03b),
        ("day04", day04::day04a, day04::day04b),
        ("day05", day05::day05a, day05::day05b),
        ("day06", day06::day06a, day06::day06b),
        ("day07", day07::day07a, day07::day07b),
        // ("day08", day08::day08a, day08::day08b),
        // ("day09", day09::day09a, day09::day09b),
        // ("day10", day10::day10a, day10::day10b),
        // ("day11", day11::day11a, day11::day11b),
        // ("day12", day12::day12a, day12::day12b),
        // ("day13", day13::day13a, day13::day13b),
        // ("day14", day14::day14a, day14::day14b),
        // ("day15", day15::day15a, day15::day15b),
        // ("day16", day16::day16a, day16::day16b),
        // ("day17", day17::day17a, day17::day17b),
        // ("day18", day18::day18a, day18::day18b),
        // ("day19", day19::day19a, day19::day19b),
        // ("day20", day20::day20a, day20::day20b),
        // ("day21", day21::day21a, day21::day21b),
        // ("day22", day22::day22a, day22::day22b),
        // ("day23", day23::day23a, day23::day23b),
        // ("day24", day24::day24a, day24::day24b),
        // ("day25", day25::day25a, day25::day25b),
    ]
}
