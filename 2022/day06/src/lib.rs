fn parse_input(input: &str) -> Vec<char> {
    return input.chars().collect();
}

fn find_marker(input: Vec<char>, marker_len: usize) -> usize {
    return input
        .windows(marker_len)
        .enumerate()
        .find(|(_, f)| {
            let mut v = f.to_vec();
            v.sort();
            v.dedup();
            return v.len() == marker_len;
        })
        .unwrap()
        .0
        + marker_len;
}

fn a(input: &str) -> usize {
    return find_marker(parse_input(input), 4);
}

fn b(input: &str) -> usize {
    return find_marker(parse_input(input), 14);
}

pub fn day06a() -> String {
    return a(include_str!("../input.txt")).to_string();
}

pub fn day06b() -> String {
    return b(include_str!("../input.txt")).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(a("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(a("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_b() {
        assert_eq!(b("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(b("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(b("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn day06() {
        assert_eq!(day06a(), "1582");
        assert_eq!(day06b(), "3588");
    }
}
