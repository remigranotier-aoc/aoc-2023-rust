use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let numbers = line.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
                let first_number = numbers.first().unwrap().to_digit(10).unwrap();
                let last_number = numbers.last().unwrap().to_digit(10).unwrap();
                (10 * first_number + last_number) as u64
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let number_regex =
        Regex::new(r"(?:[0-9]|(?:one|two|three|four|five|six|seven|eight|nine))").unwrap();
    Some(
        input
            .lines()
            .map(|line| {
                let numbers = number_regex
                    .find_iter(line)
                    .map(|number| {
                        let n = number.as_str();
                        if n.len() == 1 && n.chars().next().unwrap().is_numeric() {
                            n.parse::<u64>().unwrap()
                        } else {
                            match n {
                                "one" => 1,
                                "two" => 2,
                                "three" => 3,
                                "four" => 4,
                                "five" => 5,
                                "six" => 6,
                                "seven" => 7,
                                "eight" => 8,
                                "nine" => 9,
                                _ => unreachable!(),
                            }
                        }
                    })
                    .collect::<Vec<u64>>();
                let result = numbers.first().unwrap() * 10 + numbers.last().unwrap();
                result
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }
}
