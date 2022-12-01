fn parse_input(input: &str) -> Vec<usize> {
    return input.lines().map(|f| f.parse::<usize>().unwrap()).collect();
}

fn a(input: &str) -> usize {
    return parse_input(input).len();
}

fn b(input: &str) -> usize {
    return parse_input(input).len();
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
        assert_eq!(a(include_str!("../example.txt")), 1);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example.txt")), 1);
    }

    #[test]
    fn day00() {
        assert_eq!(day00a(), 1);
        assert_eq!(day00b(), 1);
    }
}
