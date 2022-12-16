use std::collections::HashMap;

fn find_index(input: &str, num_distinct: usize) -> u32 {
    let mut letters: HashMap<char, usize> = HashMap::new();
    for (i, ch) in input.char_indices() {
        if i > num_distinct - 1 {
            let old_letter = input.chars().nth(i - num_distinct).unwrap();
            if letters[&old_letter] == 1 {
                letters.remove(&old_letter);
            } else {
                *letters.get_mut(&old_letter).unwrap() -= 1;
            }
        }
        if let Some(count) = letters.get_mut(&ch) {
            *count += 1;
        } else {
            letters.insert(ch, 1);
        }
        if letters.len() == num_distinct {
            return i as u32 + 1;
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_index(input, 4))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_index(input, 14))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}
