use std::{
    collections::{HashMap, HashSet},
    iter,
};

fn parse_input(input: &str) -> Vec<(&str, usize)> {
    return input
        .lines()
        .map(|f| {
            let split = f.split_once(char::is_whitespace).unwrap();
            return (split.0, split.1.parse().unwrap());
        })
        .collect();
}

fn get_new_knot(head: &(isize, isize), tail: &(isize, isize)) -> ((isize, isize), bool) {
    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;

    if x_diff.abs() > 1 || y_diff.abs() > 1 {
        let mut new_knot = tail.clone();
        new_knot.0 += x_diff.signum();
        new_knot.1 += y_diff.signum();
        return (new_knot, true);
    } else {
        return (*tail, false);
    }
}

fn update_knots_and_get_new_knot(
    knots: &mut HashMap<usize, (isize, isize)>,
) -> ((isize, isize), bool) {
    for n in 1..=9 {
        let head = knots.get(&(n - 1)).unwrap();
        let tail = knots.get(&n).unwrap().clone();
        let (new_knot, moved) = get_new_knot(head, &tail);

        if moved {
            knots.insert(n, new_knot);
        }
        if n == 9 {
            return (new_knot, moved);
        }
    }
    return (knots[&0], false);
}

fn a(input: &str) -> usize {
    let mut tail_positions: Vec<(isize, isize)> = Vec::new();
    let mut head: (isize, isize) = (0, 0);
    let mut tail: (isize, isize) = (0, 0);
    tail_positions.push(tail);

    for i in parse_input(input) {
        let (direction, steps) = i;

        for _ in 0..steps {
            match direction {
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                _ => panic!("Invalid direction"),
            };

            let (new_tail, moved) = get_new_knot(&head, &tail);
            if moved {
                tail_positions.push(new_tail);
                tail = new_tail;
            }
        }
    }

    tail_positions.sort();
    tail_positions.dedup();
    return tail_positions.len();
}

fn b(input: &str) -> usize {
    let mut knots: HashMap<usize, (isize, isize)> =
        HashMap::from_iter((0..=9).zip(iter::repeat((0, 0))));

    let mut tail_positions: HashSet<(isize, isize)> = HashSet::from_iter(vec![(0, 0)]);

    for i in parse_input(input) {
        let (direction, steps) = i;

        for _ in 0..steps {
            let head = knots.get_mut(&0).unwrap();

            match direction {
                "L" => head.0 -= 1,
                "R" => head.0 += 1,
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                _ => panic!("Invalid direction"),
            };

            let (new_knot, moved) = update_knots_and_get_new_knot(&mut knots);
            if moved {
                tail_positions.insert(new_knot);
            }
        }
    }

    return tail_positions.len();
}

pub fn day09a() -> String {
    return a(include_str!("../input.txt")).to_string();
}

pub fn day09b() -> String {
    return b(include_str!("../input.txt")).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(include_str!("../example.txt")), 13);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example_2.txt")), 36);
    }

    #[test]
    fn day09() {
        assert_eq!(day09a(), "6357");
        assert_eq!(day09b(), "2627");
    }
}
