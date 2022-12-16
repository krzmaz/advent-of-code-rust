use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<String> {
    let mut input = input.lines().peekable();
    let num_stacks: usize = (input.peek().unwrap().len() + 1) / 4;
    println!("{}", num_stacks);
    let mut stack_image_buf = Vec::new();
    while !input.peek().unwrap().contains('1') {
        stack_image_buf.push(input.next().unwrap());
    }
    input.next();
    input.next();
    let mut stacks: Vec<VecDeque<&str>> = vec![VecDeque::new(); num_stacks];

    for line in stack_image_buf {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let val = line.get(i * 4 + 1..i * 4 + 2).unwrap();
            if val != " " {
                stack.push_back(val);
            }
        }
    }
    while input.peek().is_some() {
        let command: Vec<_> = input.next().unwrap().split(' ').collect();
        let amount: usize = command[1].parse().unwrap();
        let src: usize = command[3].parse::<usize>().unwrap() - 1;
        let dst: usize = command[5].parse::<usize>().unwrap() - 1;

        for _ in 0..amount {
            let val = stacks[src].pop_front().unwrap();
            stacks[dst].push_front(val);
        }
    }
    Some(stacks.iter_mut().map(|s| s.pop_front().unwrap()).collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut input = input.lines().peekable();
    let num_stacks: usize = (input.peek().unwrap().len() + 1) / 4;
    let mut stack_image_buf = Vec::new();
    while !input.peek().unwrap().contains('1') {
        stack_image_buf.push(input.next().unwrap());
    }
    input.next();
    input.next();
    let mut stacks: Vec<VecDeque<&str>> = vec![VecDeque::new(); num_stacks];

    for line in stack_image_buf {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let val = line.get(i * 4 + 1..i * 4 + 2).unwrap();
            if val != " " {
                stack.push_back(val);
            }
        }
    }
    while input.peek().is_some() {
        let command: Vec<_> = input.next().unwrap().split(' ').collect();
        let amount: usize = command[1].parse().unwrap();
        let src: usize = command[3].parse::<usize>().unwrap() - 1;
        let dst: usize = command[5].parse::<usize>().unwrap() - 1;

        let vals: Vec<_> = stacks[src].drain(..amount).collect();
        for val in vals.iter().rev() {
            stacks[dst].push_front(val);
        }
    }
    Some(stacks.iter_mut().map(|s| s.pop_front().unwrap()).collect())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
