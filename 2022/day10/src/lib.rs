use std::{collections::HashMap, iter};

fn parse_input(input: &str) -> Vec<(isize, usize)> {
    return input
        .lines()
        .rev()
        .map(|f| {
            let cycles_to_complete: usize = if f == "noop" { 1 } else { 2 };

            let x_change: isize = if f == "noop" {
                0
            } else {
                f.split_once(char::is_whitespace)
                    .unwrap()
                    .1
                    .parse::<isize>()
                    .unwrap()
            };

            return (x_change, cycles_to_complete);
        })
        .collect();
}

fn a(input: &str) -> usize {
    let mut instructions = parse_input(input);
    let mut register_x: isize = 1;

    let mut total_strength = 0;

    let (mut x_change, mut cycles_left_to_execution) = instructions.pop().unwrap();

    for cycle in 0..221 {
        cycles_left_to_execution -= 1;

        if [20, 60, 100, 140, 180, 220].contains(&(cycle + 1)) {
            total_strength += (cycle + 1) * register_x;
        }

        if cycles_left_to_execution == 0 {
            register_x += x_change;

            if instructions.is_empty() {
                break;
            }

            (x_change, cycles_left_to_execution) = instructions.pop().unwrap();
        }
    }

    return total_strength as usize;
}

fn crt_to_lines(screen: HashMap<usize, HashMap<usize, bool>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for x in 0..screen.len() {
        let row = screen.get(&x).unwrap();
        let mut line: Vec<char> = Vec::new();
        for s in 0..row.len() {
            if *row.get(&s).unwrap() {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        result.push(line.into_iter().collect());
    }
    return result;
}

fn b(input: &str) -> String {
    let mut instructions = parse_input(input);
    let mut register_x: isize = 1;

    let mut crt: HashMap<usize, HashMap<usize, bool>> = HashMap::from_iter((0..6).zip(
        iter::repeat(HashMap::from_iter((0..40).zip(iter::repeat(false)))),
    ));
    let mut sprite_position: isize = 2;

    let (mut x_change, mut cycles_left_to_execution) = instructions.pop().unwrap();

    for cycle in 0..241 {
        cycles_left_to_execution -= 1;

        let row: usize = cycle / 40;
        let col: usize = cycle % 40; // 0..=39
        let sprite_overlaps_col = (sprite_position - col as isize).abs() < 2;

        if sprite_overlaps_col {
            crt.get_mut(&row).unwrap().insert(col, true);
        }

        if cycles_left_to_execution == 0 {
            register_x += x_change;
            if x_change != 0 {
                sprite_position = register_x;
            }
            if instructions.is_empty() {
                break;
            }
            (x_change, cycles_left_to_execution) = instructions.pop().unwrap();
        }
    }

    return crt_to_lines(crt).join("\n");
}

pub fn day10a() -> String {
    return a(include_str!("../input.txt")).to_string();
}

pub fn day10b() -> String {
    return format!("\n{}", b(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(include_str!("../example.txt")), 13140);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example.txt")), ".#..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....");
    }

    #[test]
    fn day10() {
        assert_eq!(day10a(), "12740");

        // RBPARAGF
        assert_eq!(day10b(), "\n.###.###..###...##..###...##...##..####.\n#..#.#..#.#..#.#..#.#..#.#..#.#..#.#....\n#..#.###..#..#.#..#.#..#.#..#.#....###..\n###..#..#.###..####.###..####.#.##.#....\n#.#..#..#.#....#..#.#.#..#..#.#..#.#....\n#..#.###..#....#..#.#..#.#..#..###.#....");
    }
}
