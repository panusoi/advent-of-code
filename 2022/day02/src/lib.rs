fn parse_input(input: &str) -> Vec<&str> {
    return input.lines().collect();
}

fn a(input: &str) -> usize {
    return parse_input(input)
        .into_iter()
        .map(|f| {
            let opponent_play = f.chars().nth(0).unwrap();
            let my_play = f.chars().nth(2).unwrap();

            const ROCK: usize = 1;
            const PAPER: usize = 2;
            const SCISSORS: usize = 3;

            const WIN: usize = 6;
            const DRAW: usize = 3;
            const LOSE: usize = 0;

            // A & X = rock
            // B & Y = paper
            // C & Z = scissors

            if opponent_play == 'A' {
                match my_play {
                    'X' => return ROCK + DRAW,
                    'Y' => return PAPER + WIN,
                    'Z' => return SCISSORS + LOSE,
                    _ => return 0,
                }
            }

            if opponent_play == 'B' {
                match my_play {
                    'X' => return ROCK + LOSE,
                    'Y' => return PAPER + DRAW,
                    'Z' => return SCISSORS + WIN,
                    _ => return 0,
                }
            }

            if opponent_play == 'C' {
                match my_play {
                    'X' => return ROCK + WIN,
                    'Y' => return PAPER + LOSE,
                    'Z' => return SCISSORS + DRAW,
                    _ => return 0,
                }
            }

            return 0;
        })
        .sum();
}

fn b(input: &str) -> usize {
    return parse_input(input)
        .into_iter()
        .map(|f| {
            let opponent_play = f.chars().nth(0).unwrap();
            let my_play = f.chars().nth(2).unwrap();

            const ROCK: usize = 1;
            const PAPER: usize = 2;
            const SCISSORS: usize = 3;

            const WIN: usize = 6;
            const DRAW: usize = 3;
            const LOSE: usize = 0;

            // A  = rock
            // B  = paper
            // C  = scissors

            // X = lose
            // Y = draw
            // Z = win

            if opponent_play == 'A' {
                match my_play {
                    'X' => return SCISSORS + LOSE,
                    'Y' => return ROCK + DRAW,
                    'Z' => return PAPER + WIN,
                    _ => return 0,
                }
            }

            if opponent_play == 'B' {
                match my_play {
                    'X' => return ROCK + LOSE,
                    'Y' => return PAPER + DRAW,
                    'Z' => return SCISSORS + WIN,
                    _ => return 0,
                }
            }

            if opponent_play == 'C' {
                match my_play {
                    'X' => return PAPER + LOSE,
                    'Y' => return SCISSORS + DRAW,
                    'Z' => return ROCK + WIN,
                    _ => return 0,
                }
            }

            return 0;
        })
        .sum();
}

pub fn day02a() -> usize {
    return a(include_str!("../input.txt"));
}

pub fn day02b() -> usize {
    return b(include_str!("../input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(include_str!("../example.txt")), 15);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example.txt")), 12);
    }

    #[test]
    fn day02() {
        assert_eq!(day02a(), 12740);
        assert_eq!(day02b(), 11980);
    }
}
