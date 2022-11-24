fn parse_input(input: &str) -> Vec<usize> {
    return input.lines().map(|f| f.parse::<usize>().unwrap()).collect();
}

fn a(input: &str) -> usize {
    return parse_input(input)
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count();
}

fn b(input: &str) -> usize {
    return parse_input(input)
        .windows(4)
        .filter(|w| w[0] < w[3])
        .count();
}

pub fn day00a() -> usize {
    return a(include_str!("../input.txt"));
}

pub fn day00b() -> usize {
    return b(include_str!("../input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(include_str!("../example.txt")), 7);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example.txt")), 5);
    }

    #[test]
    fn day00() {
        assert_eq!(day00a(), 1374);
        assert_eq!(day00b(), 1418);
    }
}
