use std::{collections::HashSet, ops::BitAnd};

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let halves = line.split_at(line.len() / 2);
        let left = halves.0.chars().collect::<HashSet<_>>();
        let right = halves.1.chars().collect::<HashSet<_>>();
        total += left
            .intersection(&right)
            .map(|c| {
                c.clone().to_ascii_lowercase() as u32 - 'a' as u32
                    + 1
                    + if c.is_uppercase() { 26 } else { 0 }
            })
            .sum::<u32>();
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let mut iter = input.lines().peekable();
    while iter.peek().is_some() {
        let first = iter.next().unwrap().chars().collect::<HashSet<_>>();
        let second = iter.next().unwrap().chars().collect::<HashSet<_>>();
        let third = iter.next().unwrap().chars().collect::<HashSet<_>>();
        total += first
            .bitand(&second)
            .bitand(&third)
            .drain()
            .map(|c| {
                c.clone().to_ascii_lowercase() as u32 - 'a' as u32
                    + 1
                    + if c.is_uppercase() { 26 } else { 0 }
            })
            .sum::<u32>();
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
