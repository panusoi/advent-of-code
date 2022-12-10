use std::{str::Lines, usize};

struct Directory {
    size: usize,
    dirs: Vec<Directory>,
}

impl Directory {
    fn from_lines<'a>(lines: &mut Lines) -> Directory {
        let mut dir = Directory {
            size: 0,
            dirs: vec![],
        };

        while let Some(line) = lines.next() {
            if line == "$ cd .." {
                break;
            } else if line == "$ cd /" || line == "$ ls" || line.starts_with("dir") {
                continue;
            }

            // line is file or dir

            let split = line.split_once(' ').unwrap().0.parse::<usize>();
            if split.is_ok() {
                dir.size += split.unwrap();
            } else {
                dir.dirs.push(Self::from_lines(lines));
                dir.size += dir.dirs[dir.dirs.len() - 1].size;
            }
        }

        dir
    }

    fn get_all_dirs(&self) -> Vec<&Directory> {
        return self
            .dirs
            .iter()
            .flat_map(Self::get_all_dirs)
            .chain([self])
            .collect();
    }
}

fn parse_input(input: &str) -> Lines {
    return input.lines();
}

fn a(input: &str) -> usize {
    return Directory::from_lines(&mut parse_input(input))
        .get_all_dirs()
        .iter()
        .map(|dir| dir.size)
        .filter(|&size| size <= 100000)
        .sum::<usize>();
}

fn b(input: &str) -> usize {
    let root = Directory::from_lines(&mut parse_input(input));
    let required_size = 30000000 - (70000000 - root.size);
    return root
        .get_all_dirs()
        .iter()
        .map(|d| d.size)
        .filter(|&size| size >= required_size)
        .min()
        .unwrap();
}

pub fn day07a() -> String {
    return a(include_str!("../input.txt")).to_string();
}

pub fn day07b() -> String {
    return b(include_str!("../input.txt")).to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(a(include_str!("../example.txt")), 95437);
    }

    #[test]
    fn test_b() {
        assert_eq!(b(include_str!("../example.txt")), 24933642);
    }

    #[test]
    fn day07() {
        assert_eq!(day07a(), "1315285");
        assert_eq!(day07b(), "9847279");
    }
}
