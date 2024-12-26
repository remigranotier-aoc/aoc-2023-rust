use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i64> {
    let mut maps = input.split("\n\n");
    let (_, seeds) = maps.next().unwrap().split(": ").next_tuple().unwrap();
    let mut seeds = seeds
        .split(" ")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    for map in maps {
        let new_seeds = &mut seeds.to_vec();
        let (_, mapping) = map.split(":\n").next_tuple().unwrap();
        for line in mapping.lines() {
            let (target_index, source_index, source_range) = line
                .split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .next_tuple()
                .unwrap();
            for (i, seed) in seeds.iter().enumerate() {
                if *seed >= source_index && *seed < source_index + source_range {
                    new_seeds[i] = target_index + seed - source_index;
                }
            }
        }
        seeds = new_seeds.to_vec()
    }

    Some(*seeds.iter().min().unwrap())
}

fn transformed_seeds(
    seed_start: i64,
    seed_end: i64,
    mappings: &HashMap<(i64, i64), i64>,
) -> Vec<(i64, i64)> {
    // println!("Transorming range {seed_start}-{seed_end}");
    let mut transformed_seeds_result: Vec<(i64, i64)> = vec![];
    let remaining_seeds: Vec<(i64, i64)> = vec![(seed_start, seed_end)];
    for (&(range_start, range_size), &target_start) in mappings {
        let range_end = range_start + range_size - 1;
        let offset = target_start - range_start;
        // println!("Watching range {range_start}-{range_end}");
        if range_start <= seed_start {
            if range_end < seed_start {
                continue;
            } else if range_end >= seed_end {
                transformed_seeds_result.push((seed_start + offset, seed_end + offset));
                break;
            } else {
                transformed_seeds_result.push((seed_start + offset, range_end + offset));
                transformed_seeds_result.append(&mut transformed_seeds(
                    range_end + 1,
                    seed_end,
                    mappings,
                ));
            }
        } else if range_start > seed_end {
            continue;
        } else if range_end >= seed_end {
            transformed_seeds_result.push((range_start + offset, seed_end + offset));
            transformed_seeds_result.append(&mut transformed_seeds(
                seed_start,
                range_start - 1,
                mappings,
            ));
        } else {
            transformed_seeds_result.push((range_start + offset, range_end + offset));
            transformed_seeds_result.append(&mut transformed_seeds(
                seed_start,
                range_start - 1,
                mappings,
            ));
            transformed_seeds_result.append(&mut transformed_seeds(
                range_end + 1,
                seed_end,
                mappings,
            ));
        }
    }

    if transformed_seeds_result.is_empty() {
        return remaining_seeds;
    }

    transformed_seeds_result
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut maps = input.split("\n\n");
    let (_, seed_ranges) = maps.next().unwrap().split(": ").next_tuple().unwrap();
    let seed_ranges = seed_ranges
        .split(" ")
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut seeds: Vec<(i64, i64)> = vec![];
    for (initial_seed, size) in seed_ranges.into_iter().tuples() {
        seeds.push((initial_seed, initial_seed + size - 1));
    }
    // println!("{seeds:?}");

    for map in maps {
        let mut new_seeds: Vec<(i64, i64)> = vec![];

        let (_, mapping) = map.split(":\n").next_tuple().unwrap();
        let mut mappings: HashMap<(i64, i64), i64> = HashMap::new();
        for line in mapping.lines() {
            let (target_index, source_index, source_range) = line
                .split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .next_tuple()
                .unwrap();
            mappings.insert((source_index, source_range), target_index);
        }

        for (seed_start, seed_end) in seeds {
            new_seeds.append(&mut transformed_seeds(seed_start, seed_end, &mappings));
        }

        seeds = new_seeds;
        // println!(" ===== {seeds:?}");
    }

    Some(seeds.iter().map(|tuple| tuple.0).min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
