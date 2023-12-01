use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n")
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let parsed: Vec<_> = entry
                .chars()
                .filter_map(|char| {
                    match String::from(char.clone()).parse::<u32>() {
                        Ok(number) => Some(char),
                        _ => None
                    }
                })
                .collect();

            let chars = vec![parsed[0].to_string(), parsed.last().unwrap().to_string()];

            Some(chars.join("").parse::<u32>().unwrap())
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .split("\n")
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            entry
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .map(|entry| {
            let parsed: Vec<_> = entry
                .chars()
                .filter_map(|char| {
                    match String::from(char.clone()).parse::<u32>() {
                        Ok(number) => Some(char),
                        _ => None
                    }
                })
                .collect();

            let chars = vec![parsed[0].to_string(), parsed.last().unwrap().to_string()];

            Some(chars.join("").parse::<u32>().unwrap())
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::Day;
    use std::env;
    use std::fs;

    pub fn read_file_part(folder: &str, day: Day, part: u8) -> String {
        let cwd = env::current_dir().unwrap();
        let filepath = cwd.join("data").join(folder).join(format!("{day}-{part}.txt"));
        let f = fs::read_to_string(filepath);
        f.expect("could not open input file")
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}
