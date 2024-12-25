use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|line| {
                let (_, numbers) = line.split(": ").next_tuple().unwrap();
                let (wished_numbers, card_numbers) = numbers.split(" | ").next_tuple().unwrap();
                let wished_numbers = wished_numbers
                    .split(" ")
                    .filter(|n| !n.is_empty())
                    .map(|n| n.trim().parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                let card_numbers = card_numbers
                    .split(" ")
                    .filter(|n| !n.is_empty())
                    .map(|n| n.trim().parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                let numbers_found = wished_numbers
                    .iter()
                    .filter(|n| card_numbers.contains(n))
                    .count() as u32;
                if numbers_found > 0 {
                    2_u64.pow(numbers_found - 1)
                } else {
                    0
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut scratchcards: HashMap<u64, u64> = HashMap::new();

    input.lines().for_each(|line| {
        let (card_name, numbers) = line.split(": ").next_tuple().unwrap();
        let (_, card_id) = card_name
            .split(" ")
            .filter(|word| !word.is_empty())
            .next_tuple()
            .unwrap();
        let card_id = card_id.parse::<u64>().unwrap();
        let current_entry = scratchcards.entry(card_id).or_default();
        *current_entry += 1;
        let (wished_numbers, card_numbers) = numbers.split(" | ").next_tuple().unwrap();
        let wished_numbers = wished_numbers
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let card_numbers = card_numbers
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let numbers_found = wished_numbers
            .iter()
            .filter(|n| card_numbers.contains(n))
            .count() as u32;
        for i in 1..=numbers_found {
            *scratchcards.entry(card_id + i as u64).or_default() +=
                *scratchcards.get(&card_id).unwrap();
        }
    });

    Some(scratchcards.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
