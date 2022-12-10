fn parse_input(input: &str) -> Vec<Vec<usize>> {
    return input
        .lines()
        .map(|f| {
            f.chars()
                .into_iter()
                .map(|f| f.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
}

fn a(input: &str) -> usize {
    let parsed_input = parse_input(input);
    let max_i: usize = parsed_input.len();

    let mut visible: usize = 0;

    for i in 0..max_i {
        let vec_i = &parsed_input[i];
        let max_j: usize = vec_i.len();

        for j in 0..max_j {
            let j_val = vec_i[j];

            if i == 0 || i == max_i - 1 || j == 0 || j == max_j - 1 {
                visible += 1;
                continue;
            }

            let mut up = true;
            let mut down = true;
            let mut left = true;
            let mut right = true;

            for i_up in i + 1..max_i {
                if j_val > parsed_input[i_up][j] {
                    visible += 0;
                } else {
                    up = false;
                    break;
                }
            }

            for i_down in 0..i {
                if j_val > parsed_input[i_down][j] {
                    visible += 0;
                } else {
                    down = false;
                    break;
                }
            }

            for j_right in j + 1..max_j {
                if j_val > parsed_input[i][j_right] {
                    visible += 0;
                } else {
                    right = false;
                    break;
                }
            }

            for j_left in 0..j {
                if j_val > parsed_input[i][j_left] {
                    visible += 0;
                } else {
                    left = false;
                    break;
                }
            }

            if up || down || left || right {
                visible += 1;
            }
        }
    }

    return visible;
}

fn b(input: &str) -> usize {
    let parsed_input = parse_input(input);
    let max_i: usize = parsed_input.len();

    let mut max: usize = 0;

    for i in 0..max_i {
        let vec_i = &parsed_input[i];

        let max_j: usize = vec_i.len();

        for j in 0..max_j {
            let j_val = vec_i[j];

            if i == 0 || i == max_i - 1 || j == 0 || j == max_j - 1 {
                continue;
            }

            let mut up_tree = 0;
            let mut down_tree = 0;
            let mut right_tree = 0;
            let mut left_tree = 0;

            for i_down in i + 1..max_i {
                if j_val > parsed_input[i_down][j] {
                    down_tree += 1;
                } else if j_val == parsed_input[i_down][j] {
                    down_tree += 1;
                    break;
                } else {
                    down_tree += 1;
                    break;
                }
            }

            for i_up in (0..i).rev() {
                if j_val > parsed_input[i_up][j] {
                    up_tree += 1;
                } else if j_val == parsed_input[i_up][j] {
                    up_tree += 1;
                    break;
                } else {
                    up_tree += 1;
                    break;
                }
            }

            for j_right in j + 1..max_j {
                if j_val > parsed_input[i][j_right] {
                    right_tree += 1;
                } else if j_val == parsed_input[i][j_right] {
                    right_tree += 1;
                    break;
                } else {
                    right_tree += 1;
                    break;
                }
            }

            for j_left in (0..j).rev() {
                if j_val > parsed_input[i][j_left] {
                    left_tree += 1;
                } else if j_val == parsed_input[i][j_left] {
                    left_tree += 1;
                    break;
                } else {
                    left_tree += 1;
                    break;
                }
            }

            let result = left_tree * right_tree * up_tree * down_tree;

            if result > max {
                max = result;
            }
        }
    }

    return max;
}

pub fn day08a() -> String {
    return a(include_str!("../input.txt")).to_string();
}

pub fn day08b() -> String {
    return b(include_str!("../input.txt")).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(include_str!("../example.txt")), 21);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example.txt")), 8);
    }

    #[test]
    fn day08() {
        assert_eq!(day08a(), "1533");
        assert_eq!(day08b(), "345744");
    }
}
