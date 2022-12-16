pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let vals: Vec<_> = line
            .split(',')
            .flat_map(|s| s.split('-'))
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        if (vals[0] <= vals[2] && vals[1] >= vals[3]) || (vals[0] >= vals[2] && vals[1] <= vals[3])
        {
            total += 1;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let vals: Vec<_> = line
            .split(',')
            .flat_map(|s| s.split('-'))
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        if (vals[0] <= vals[3] && vals[1] >= vals[3])
            || (vals[0] <= vals[2] && vals[1] >= vals[2])
            || (vals[2] <= vals[0] && vals[0] <= vals[3])
            || (vals[2] <= vals[1] && vals[1] <= vals[3])
        {
            total += 1;
        }
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
