use itertools::Itertools;

advent_of_code::solution!(2);

const MAX_RED: u64 = 12;
const MAX_GREEN: u64 = 13;
const MAX_BLUE: u64 = 14;

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|game| {
                let (game_title, draws) = game.split(": ").next_tuple().unwrap();
                let game_id = game_title
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                let reveals: Vec<&str> = draws.split("; ").collect::<Vec<_>>();
                reveals
                    .iter()
                    .all(|reveal| {
                        let indiv_reveals: Vec<&str> = reveal.split(", ").collect::<Vec<_>>();
                        indiv_reveals.iter().all(|indiv_reveal| {
                            let (amount, color) = indiv_reveal.split(" ").next_tuple().unwrap();
                            let amount = amount.parse::<u64>().unwrap();
                            match color {
                                "blue" => amount <= MAX_BLUE,
                                "red" => amount <= MAX_RED,
                                "green" => amount <= MAX_GREEN,
                                _ => unreachable!(),
                            }
                        })
                    })
                    .then_some(game_id)
            })
            .sum::<u64>(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|game| {
                let (mut max_red, mut max_blue, mut max_green) = (0, 0, 0);
                let (_, draws) = game.split(": ").next_tuple().unwrap();
                let reveals: Vec<&str> = draws.split("; ").collect::<Vec<_>>();
                reveals.iter().for_each(|reveal| {
                    let indiv_reveals: Vec<&str> = reveal.split(", ").collect::<Vec<_>>();
                    indiv_reveals.iter().for_each(|indiv_reveal| {
                        let (amount, color) = indiv_reveal.split(" ").next_tuple().unwrap();
                        let amount = amount.parse::<u64>().unwrap();
                        match color {
                            "blue" => {
                                if amount > max_blue {
                                    max_blue = amount;
                                }
                            }
                            "red" => {
                                if amount > max_red {
                                    max_red = amount;
                                }
                            }
                            "green" => {
                                if amount > max_green {
                                    max_green = amount;
                                }
                            }
                            _ => unreachable!(),
                        }
                    })
                });
                max_blue * max_red * max_green
            })
            .sum::<u64>(),
    )
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
