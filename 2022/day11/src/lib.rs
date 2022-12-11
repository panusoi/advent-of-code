use std::vec;

struct MonkeyNote {
    items: Vec<usize>,
    operation: (String, String, String),
    test: usize,
    test_true_target: usize,
    test_false_target: usize,
    items_inspected: usize,
}

fn parse_monkey_note(monkey_note_vec: Vec<&str>) -> MonkeyNote {
    if monkey_note_vec.len() != 6 {
        panic!("Invalid note");
    }

    let items: Vec<usize> = monkey_note_vec[1]
        .replace("Starting items: ", "")
        .split(",")
        .map(|f| f.trim().parse().unwrap())
        .collect();

    let binding = monkey_note_vec[2].replace("Operation: new = ", "");
    let operation_vec: Vec<&str> = binding.split_whitespace().collect();

    if operation_vec.len() != 3 {
        panic!("Invalid note operation")
    }

    let operation: (String, String, String) = (
        operation_vec[0].to_string(),
        operation_vec[1].to_string(),
        operation_vec[2].to_string(),
    );

    let test: usize = monkey_note_vec[3]
        .replace("Test: divisible by ", "")
        .parse()
        .unwrap();

    let test_true_target: usize = monkey_note_vec[4]
        .replace("If true: throw to monkey ", "")
        .parse()
        .unwrap();

    let test_false_target: usize = monkey_note_vec[5]
        .replace("If false: throw to monkey ", "")
        .parse()
        .unwrap();

    return MonkeyNote {
        items,
        operation,
        test,
        test_true_target,
        test_false_target,
        items_inspected: 0,
    };
}

fn parse_input(input: &str) -> Vec<MonkeyNote> {
    let mut monkey_notes: Vec<MonkeyNote> = vec![];
    let mut temp_monkey_note: Vec<&str> = vec![];

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line.trim() != "" {
            temp_monkey_note.push(line.trim());
        } else {
            monkey_notes.push(parse_monkey_note(temp_monkey_note));
            temp_monkey_note = vec![];
        }
    }

    monkey_notes.push(parse_monkey_note(temp_monkey_note));

    return monkey_notes;
}

/**
 * Does some monkey business and returns a tuple vector containing (target monkey, item)
 */
fn do_monkey_business(monkey_note: &mut MonkeyNote, lcm: usize) -> Vec<(usize, usize)> {
    let monkey_operation = &monkey_note.operation;
    let monkey_items: &mut Vec<usize> = monkey_note.items.as_mut();
    let test = monkey_note.test;
    let test_true_target = monkey_note.test_true_target;
    let test_false_target = monkey_note.test_false_target;

    let mut result: Vec<(usize, usize)> = vec![];

    for _ in 0..monkey_items.len() {
        monkey_note.items_inspected += 1;
        let mut item = monkey_items.remove(0);

        let val_1: usize = if monkey_operation.0 == "old" {
            item
        } else {
            monkey_operation.0.parse().unwrap()
        };

        let val_2: usize = if monkey_operation.2 == "old" {
            item
        } else {
            monkey_operation.2.parse().unwrap()
        };

        let new_value = match monkey_operation.1.as_str() {
            "+" => val_1 + val_2,
            "*" => val_1 * val_2,
            _ => panic!("Invalid operation marker"),
        };

        if lcm == 0 {
            item = new_value / 3;
        } else {
            item = new_value % lcm;
        }

        if item % test == 0 {
            result.insert(0, (test_true_target, item));
        } else {
            result.insert(0, (test_false_target, item));
        }
    }

    return result;
}

/**
 * Returns level of monkey business
 */
fn let_monkeys_loose(input: &str, rounds: usize, use_lcm: bool) -> usize {
    let mut monkey_notes = parse_input(input);

    //  lowest common multiple
    let lcm = if use_lcm {
        monkey_notes
            .iter()
            .fold(1 as usize, |acc, item| acc * item.test)
    } else {
        0
    };

    for round in 0..rounds * monkey_notes.len() {
        let monkey = round % monkey_notes.len();

        let monkey_business = do_monkey_business(monkey_notes.get_mut(monkey).unwrap(), lcm);

        for i in 0..monkey_business.len() {
            let (monkey, item) = monkey_business[i];
            let monkey_note = monkey_notes.get_mut(monkey).unwrap();
            let items: &mut Vec<usize> = monkey_note.items.as_mut();
            items.push(item);
        }
    }

    let mut inspections: Vec<usize> = monkey_notes.iter().map(|f| f.items_inspected).collect();
    inspections.sort();
    return inspections[inspections.len() - 1] * inspections[inspections.len() - 2];
}

fn a(input: &str) -> usize {
    return let_monkeys_loose(input, 20, false);
}

fn b(input: &str) -> usize {
    return let_monkeys_loose(input, 10000, true);
}

pub fn day11a() -> String {
    return a(include_str!("../input.txt")).to_string();
}

pub fn day11b() -> String {
    return b(include_str!("../input.txt")).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(include_str!("../example.txt")), 10605);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example.txt")), 2713310158);
    }

    #[test]
    fn day11() {
        assert_eq!(day11a(), "66124");
        assert_eq!(day11b(), "19309892877");
    }
}
