use std::{collections::HashMap, path::PathBuf};

pub fn part_one(input: &str) -> Option<u32> {
    let mut cwd = PathBuf::from("/");
    let mut all_file_sizes = HashMap::new();
    for line in input.lines() {
        // handle commands
        if line.starts_with('$') {
            match line {
                "$ cd /" => {
                    cwd = PathBuf::from("/");
                }
                "$ cd .." => {
                    cwd.pop();
                }
                "$ ls" => {
                    // ignore, we'll handle the following lines later
                }
                _ => {
                    // this must be cd into a new directory, so we change cwd
                    let dir = line.split(' ').nth(2).unwrap();
                    cwd = cwd.join(dir);
                }
            }
        }
        // handle output of ls
        let parts: Vec<_> = line.split(' ').collect();
        // number on the first
        if let Ok(size) = parts[0].parse::<u32>() {
            let name = parts[1].to_string();
            all_file_sizes.insert(cwd.join(name).display().to_string(), size);
        }
    }
    let mut directory_sizes = HashMap::new();
    for (path, size) in all_file_sizes {
        let path = PathBuf::from(path);
        for ancestor in path.ancestors().skip(1) {
            let ancestor = ancestor.to_path_buf();
            if let Some(old_size) = directory_sizes.get(&ancestor) {
                directory_sizes.insert(ancestor, size + old_size);
            } else {
                directory_sizes.insert(ancestor, size);
            }
        }
    }
    let mut total: u32 = 0;
    for (_, size) in directory_sizes {
        if size < 100000 {
            total += size;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cwd = PathBuf::from("/");
    let mut all_file_sizes = HashMap::new();
    for line in input.lines() {
        // handle commands
        if line.starts_with('$') {
            match line {
                "$ cd /" => {
                    cwd = PathBuf::from("/");
                }
                "$ cd .." => {
                    cwd.pop();
                }
                "$ ls" => {
                    // ignore, we'll handle the following lines later
                }
                _ => {
                    let dir = line.split(' ').nth(2).unwrap();
                    cwd = cwd.join(dir);
                }
            }
        }
        // handle output of ls
        let parts: Vec<_> = line.split(' ').collect();
        // number on the first position means file size
        if let Ok(size) = parts[0].parse::<u32>() {
            all_file_sizes.insert(cwd.join(parts[1]).display().to_string(), size);
        }
    }
    let mut directory_sizes = HashMap::new();
    for (path, size) in all_file_sizes {
        let path = PathBuf::from(path);
        for ancestor in path.ancestors().skip(1) {
            let ancestor = ancestor.to_path_buf();
            if let Some(old_size) = directory_sizes.get(&ancestor) {
                directory_sizes.insert(ancestor, size + old_size);
            } else {
                directory_sizes.insert(ancestor, size);
            }
        }
    }

    let mut directory_sizes: Vec<_> = directory_sizes.iter().collect();
    directory_sizes.sort_by(|a, b| a.1.cmp(b.1));
    let currently_free = 70000000_u32 - directory_sizes.last().unwrap().1;
    let target = 30000000_u32 - currently_free;

    Some(
        *directory_sizes
            .iter()
            .find(|(_, size)| size >= &&target)
            .unwrap()
            .1,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
