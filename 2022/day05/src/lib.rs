fn add_to_crate_stack(crate_stacks: &mut Vec<Vec<char>>, crate_stack_index: usize, value: char) {
    while crate_stacks.len() - 1 < crate_stack_index {
        crate_stacks.push(Vec::new());
    }
    crate_stacks
        .get_mut(crate_stack_index)
        .unwrap()
        .insert(0, value);
}

fn get_crate_char(value: String) -> char {
    return value.chars().find(|c| c.is_alphabetic()).unwrap();
}

fn get_crate_move(value: &str) -> (usize, usize, usize) {
    let move_vec: Vec<usize> = value
        .replace("move ", "")
        .replace("from ", "")
        .replace("to ", "")
        .split(' ')
        .into_iter()
        .map(|f| f.parse().unwrap())
        .collect();

    return (move_vec[0], move_vec[1], move_vec[2]);
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    return input.lines().fold(
        (Vec::new(), Vec::new()),
        |(mut crate_stacks, mut crate_moves), f| {
            if crate_stacks.is_empty() {
                crate_stacks.push(Vec::new());
            }

            if !f.starts_with("move") && f.contains("[") {
                f.chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|c| c.iter().collect::<String>())
                    .enumerate()
                    .for_each(|(index, value)| {
                        if value.trim() != "" {
                            add_to_crate_stack(crate_stacks.as_mut(), index, get_crate_char(value));
                        }
                    });
            } else if f.starts_with("move") {
                crate_moves.push(get_crate_move(f));
            }

            return (crate_stacks, crate_moves);
        },
    );
}

fn move_crates(
    crate_stacks: &mut Vec<Vec<char>>,
    (count, from, to): (usize, usize, usize),
    move_one_by_one: bool,
) {
    let mut total_moved: usize = 0;

    while total_moved < count {
        let from_stack = crate_stacks.get_mut(from - 1).unwrap();

        let take_crates_from: Vec<char> = if move_one_by_one {
            let start = from_stack.len() - 1;
            from_stack.drain(start..start + 1).collect()
        } else {
            let start = from_stack.len() - count + total_moved;
            from_stack.drain(start..).collect()
        };

        let to_stack = crate_stacks.get_mut(to - 1).unwrap();
        to_stack.extend(take_crates_from.iter());
        total_moved += take_crates_from.len();
    }
}

fn get_crate_tops(crate_stacks: &Vec<Vec<char>>) -> Vec<char> {
    return crate_stacks
        .into_iter()
        .filter(|f| !f.is_empty())
        .map(|f| f.clone().pop().unwrap())
        .collect();
}

fn a(input: &str) -> String {
    let (mut crate_stacks, crate_moves) = parse_input(input);
    crate_moves.into_iter().for_each(|crate_move| {
        move_crates(crate_stacks.as_mut(), crate_move, true);
    });
    return get_crate_tops(&crate_stacks).into_iter().collect();
}

fn b(input: &str) -> String {
    let (mut crate_stacks, crate_moves) = parse_input(input);
    crate_moves.into_iter().for_each(|crate_move| {
        move_crates(crate_stacks.as_mut(), crate_move, false);
    });
    return get_crate_tops(&crate_stacks).into_iter().collect();
}

pub fn day05a() -> String {
    return a(include_str!("../input.txt"));
}

pub fn day05b() -> String {
    return b(include_str!("../input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(include_str!("../example.txt")), "CMZ");
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example.txt")), "MCD");
    }

    #[test]
    fn day05() {
        assert_eq!(day05a(), "DHBJQJCCW");
        assert_eq!(day05b(), "WJVRLSJJT");
    }
}
