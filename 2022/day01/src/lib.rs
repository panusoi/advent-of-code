// returns the total calories for each elf in vec
fn parse_input_to_sums(input: &str) -> Vec<usize> {
    let sums = input
        .lines()
        .fold(Vec::new(), |mut acc, f| {
            if acc.is_empty() {
                acc.push(Vec::new());
            }

            // store each elf's calories in their own vec
            if f == "" {
                acc.push(Vec::new());
            } else {
                acc.last_mut().unwrap().push(f.parse::<usize>().unwrap());
            }
            return acc;
        })
        .iter()
        // sum the elf's calories
        .map(|f| f.iter().sum::<usize>())
        .collect::<Vec<usize>>();
    return sums;
}

fn a(input: &str) -> usize {
    return *parse_input_to_sums(input).iter().max().unwrap();
}

fn b(input: &str) -> usize {
    let mut sums: Vec<usize> = parse_input_to_sums(input);
    sums.sort();
    let top3 = *sums.get(sums.len() - 1).unwrap()
        + *sums.get(sums.len() - 2).unwrap()
        + *sums.get(sums.len() - 3).unwrap();
    return top3;
}

pub fn day01a() -> String {
    return a(include_str!("../input.txt")).to_string();
}

pub fn day01b() -> String {
    return b(include_str!("../input.txt")).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(include_str!("../example.txt")), 24000);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example.txt")), 45000);
    }

    #[test]
    fn day01() {
        assert_eq!(day01a(), "72070");
        assert_eq!(day01b(), "211805");
    }
}
