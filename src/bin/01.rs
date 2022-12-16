use std::{cmp::Reverse, collections::BinaryHeap};

pub fn part_one(input: &str) -> Option<u32> {
    let mut max_sum: u32 = 0;
    let mut current_sum: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            if current_sum > max_sum {
                max_sum = current_sum;
            }
            current_sum = 0;
            continue;
        }
        let value: u32 = line.parse().unwrap();
        current_sum += value;
    }
    Some(max_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut heap: BinaryHeap<Reverse<u32>> = BinaryHeap::with_capacity(3);
    let mut current_sum: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            if heap.len() < 3 || current_sum > heap.peek().unwrap().0 {
                heap.push(Reverse(current_sum));
                if heap.len() > 3 {
                    heap.pop();
                }
            }
            current_sum = 0;
            continue;
        }
        let value: u32 = line.parse().unwrap();
        current_sum += value;
    }
    Some(heap.iter().map(|x| x.0).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
