fn parse_input(input: &str) -> Vec<((usize, usize), (usize, usize))> {
    return input
        .lines()
        .map(|f| {
            let pairs: Vec<&str> = f.split(",").collect();
            let first_pair_section: Vec<usize> = pairs
                .get(0)
                .unwrap()
                .split("-")
                .map(|f| f.parse::<usize>().unwrap())
                .collect();

            let first_pair_section_tuple: (usize, usize) = (
                *first_pair_section.get(0).unwrap(),
                *first_pair_section.get(1).unwrap(),
            );

            let second_pair_section: Vec<usize> = pairs
                .get(1)
                .unwrap()
                .split("-")
                .map(|f| f.parse::<usize>().unwrap())
                .collect();

            let second_pair_section_tuple: (usize, usize) = (
                *second_pair_section.get(0).unwrap(),
                *second_pair_section.get(1).unwrap(),
            );

            return (first_pair_section_tuple, second_pair_section_tuple);
        })
        .collect();
}

fn is_range_in_range(range_1: (usize, usize), range_2: (usize, usize)) -> bool {
    return range_1.0 >= range_2.0 && range_1.1 <= range_2.1;
}

fn is_value_in_range(value: usize, range: (usize, usize)) -> bool {
    return value >= range.0 && value <= range.1;
}

fn a(input: &str) -> usize {
    return parse_input(input)
        .into_iter()
        .map(|pair_sections| {
            if is_range_in_range(pair_sections.0, pair_sections.1)
                || is_range_in_range(pair_sections.1, pair_sections.0)
            {
                return 1;
            }

            return 0;
        })
        .sum();
}

fn b(input: &str) -> usize {
    return parse_input(input)
        .into_iter()
        .map(|pair_sections| {
            if is_value_in_range(pair_sections.0 .0, pair_sections.1)
                || is_value_in_range(pair_sections.0 .1, pair_sections.1)
                || is_value_in_range(pair_sections.1 .0, pair_sections.0)
                || is_value_in_range(pair_sections.1 .1, pair_sections.0)
            {
                return 1;
            }

            return 0;
        })
        .sum();
}

pub fn day04a() -> usize {
    return a(include_str!("../input.txt"));
}

pub fn day04b() -> usize {
    return b(include_str!("../input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(include_str!("../example.txt")), 2);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example.txt")), 4);
    }

    #[test]
    fn day04() {
        assert_eq!(day04a(), 605);
        assert_eq!(day04b(), 914);
    }
}
