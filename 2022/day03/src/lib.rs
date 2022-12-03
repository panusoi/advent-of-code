fn parse_input(input: &str) -> Vec<&str> {
    return input.lines().collect();
}
fn a(input: &str) -> usize {
    let rucksacks = parse_input(input);

    let result = rucksacks.into_iter().map(|f| {
        let (first_compartment, second_compartment) = f.split_at(f.len() / 2);

        let first_item_in_both = first_compartment
            .chars()
            .find(|f| second_compartment.chars().find(|f2| f == f2).is_some())
            .unwrap();

        const CHAR_LOWER_A_VALUE: usize = 96;
        const CHAR_UPPER_A_VALUE: usize = 38;

        return match first_item_in_both.is_uppercase() {
            true => first_item_in_both as usize - CHAR_UPPER_A_VALUE,
            false => first_item_in_both as usize - CHAR_LOWER_A_VALUE,
        };
    });

    return result.sum();
}

fn b(input: &str) -> usize {
    return parse_input(input)
        .chunks(3)
        .into_iter()
        .map(|f| {
            let first_rucksack = f.get(0).unwrap();
            let second_rucksack = f.get(1).unwrap();
            let third_rucksack = f.get(2).unwrap();

            let first_item_in_all = first_rucksack
                .chars()
                .find(|f| {
                    second_rucksack.chars().find(|f2| f == f2).is_some()
                        && third_rucksack.chars().find(|f2| f == f2).is_some()
                })
                .unwrap();

            const CHAR_LOWER_A_VALUE: usize = 96;
            const CHAR_UPPER_A_VALUE: usize = 38;

            return match first_item_in_all.is_uppercase() {
                true => first_item_in_all as usize - CHAR_UPPER_A_VALUE,
                false => first_item_in_all as usize - CHAR_LOWER_A_VALUE,
            };
        })
        .sum();
}

pub fn day03a() -> usize {
    return a(include_str!("../input.txt"));
}

pub fn day03b() -> usize {
    return b(include_str!("../input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(include_str!("../example.txt")), 157);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example.txt")), 70);
    }

    #[test]
    fn day03() {
        assert_eq!(day03a(), 7908);
        assert_eq!(day03b(), 2838);
    }
}
