advent_of_code::solution!(2);

use regex::Regex;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

struct Game {
    id: u32,
    rounds: Vec<GameRound>
}

impl Game {
    fn from_string(input: &str) -> Game {
        let re = Regex::new(r"Game (\d+): (.+)$").unwrap();
        let captures = re.captures(input).unwrap();

        let id = captures[1].parse::<u32>().unwrap();
        let rounds: Vec<GameRound> = captures[2]
            .split(';')
            .map(|round| GameRound::from_string(round))
            .collect();

        Game { id, rounds }
    }

    fn is_valid(&self) -> bool {
        self.rounds
            .iter()
            .all(|round| round.is_valid())
    }

    fn power_of_minimum(&self) -> u32 {
        let result = self.minimum_cubes();

        result.0 * result.1 * result.2
    }

    fn minimum_cubes(&self) -> (u32, u32, u32) {
        self
            .rounds
            .iter()
            .fold((0, 0, 0), |acc, round| {
                (
                    *vec![acc.0, round.red].iter().max().unwrap(),
                    *vec![acc.1, round.green].iter().max().unwrap(),
                    *vec![acc.2, round.blue].iter().max().unwrap(),
                    )
            })
    }
}

struct GameRound {
    green: u32,
    red: u32,
    blue: u32
}

impl GameRound {
    pub fn from_string(input: &str) -> GameRound {
        let mut green = 0;
        let mut red = 0;
        let mut blue = 0;

        input
            .split(',')
            .for_each(|entry| {
                let re = Regex::new(r"(\d+) (green|blue|red)$").unwrap();
                let captures = re.captures(entry).unwrap();

                let color = &captures[2];
                let count = captures[1].parse::<u32>().unwrap();

                match color {
                    "green" => green += count,
                    "blue" => blue += count,
                    "red" => red += count,
                    _ => {}
                }
            });

        GameRound { red, green, blue }
    }

    fn is_valid(&self) -> bool {
        (self.red <= MAX_RED) && (self.blue <= MAX_BLUE) && (self.green <= MAX_GREEN)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n")
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let game = Game::from_string(entry);

            match game.is_valid() {
                true => Some(game.id),
                false => Some(0)
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .split("\n")
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let game = Game::from_string(entry);
            let result = game.power_of_minimum();

            Some(result)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
