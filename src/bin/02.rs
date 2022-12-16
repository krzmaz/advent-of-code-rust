use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let scores = HashMap::from([
        ("A X", 4),
        ("B X", 1),
        ("C X", 7),
        ("A Y", 8),
        ("B Y", 5),
        ("C Y", 2),
        ("A Z", 3),
        ("B Z", 9),
        ("C Z", 6),
    ]);
    let mut total: u32 = 0;
    for line in input.lines() {
        total += scores[line];
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let scores = HashMap::from([
        ("A X", 3),
        ("B X", 1),
        ("C X", 2),
        ("A Y", 4),
        ("B Y", 5),
        ("C Y", 6),
        ("A Z", 8),
        ("B Z", 9),
        ("C Z", 7),
    ]);
    let mut total: u32 = 0;
    for line in input.lines() {
        total += scores[line];
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
