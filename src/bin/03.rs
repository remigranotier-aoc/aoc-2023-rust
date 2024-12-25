use std::collections::HashMap;

advent_of_code::solution!(3);

fn is_symbol_nearby(
    text: &[Vec<char>],
    line: usize,
    start_index: usize,
    end_index: usize,
    stars: Option<&mut HashMap<(usize, usize), Vec<u64>>>,
) -> bool {
    let v_size = text.len();
    let h_size = text[0].len();
    if start_index > 0 {
        let tested_char = text[line][start_index - 1];
        if !tested_char.is_numeric() && tested_char != '.' {
            if stars.is_some() && tested_char == '*' {
                let number = text[line][start_index..=end_index]
                    .iter()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                stars
                    .unwrap()
                    .entry((line, start_index - 1))
                    .or_default()
                    .push(number);
            }
            return true;
        }
        if line > 0 {
            let tested_char = text[line - 1][start_index - 1];
            if !tested_char.is_numeric() && tested_char != '.' {
                if stars.is_some() && tested_char == '*' {
                    let number = text[line][start_index..=end_index]
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                    stars
                        .unwrap()
                        .entry((line - 1, start_index - 1))
                        .or_default()
                        .push(number);
                }
                return true;
            }
        }
        if line < v_size - 1 {
            let tested_char = text[line + 1][start_index - 1];
            if !tested_char.is_numeric() && tested_char != '.' {
                if stars.is_some() && tested_char == '*' {
                    let number = text[line][start_index..=end_index]
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                    stars
                        .unwrap()
                        .entry((line + 1, start_index - 1))
                        .or_default()
                        .push(number);
                }
                return true;
            }
        }
    }

    if line > 0 {
        for j in start_index..=end_index {
            let tested_char = text[line - 1][j];
            if !tested_char.is_numeric() && tested_char != '.' {
                if stars.is_some() && tested_char == '*' {
                    let number = text[line][start_index..=end_index]
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                    stars
                        .unwrap()
                        .entry((line - 1, j))
                        .or_default()
                        .push(number);
                }
                return true;
            }
        }
    }

    if line < v_size - 1 {
        for j in start_index..=end_index {
            let tested_char = text[line + 1][j];
            if !tested_char.is_numeric() && tested_char != '.' {
                if stars.is_some() && tested_char == '*' {
                    let number = text[line][start_index..=end_index]
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                    stars
                        .unwrap()
                        .entry((line + 1, j))
                        .or_default()
                        .push(number);
                }
                return true;
            }
        }
    }

    if end_index < h_size - 1 {
        let tested_char = text[line][end_index + 1];
        if !tested_char.is_numeric() && tested_char != '.' {
            if stars.is_some() && tested_char == '*' {
                let number = text[line][start_index..=end_index]
                    .iter()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                stars
                    .unwrap()
                    .entry((line, end_index + 1))
                    .or_default()
                    .push(number);
            }
            return true;
        }
        if line > 0 {
            let tested_char = text[line - 1][end_index + 1];
            if !tested_char.is_numeric() && tested_char != '.' {
                if stars.is_some() && tested_char == '*' {
                    let number = text[line][start_index..=end_index]
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                    stars
                        .unwrap()
                        .entry((line - 1, end_index + 1))
                        .or_default()
                        .push(number);
                }
                return true;
            }
        }
        if line < v_size - 1 {
            let tested_char = text[line + 1][end_index + 1];
            if !tested_char.is_numeric() && tested_char != '.' {
                if stars.is_some() && tested_char == '*' {
                    let number = text[line][start_index..=end_index]
                        .iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                    stars
                        .unwrap()
                        .entry((line + 1, end_index + 1))
                        .or_default()
                        .push(number);
                }
                return true;
            }
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let text: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let h_size = text[0].len();
    let mut total = 0;
    for (i, line) in text.iter().enumerate() {
        let mut current_number: String = String::new();
        let mut start_index: usize = 0;
        let mut end_index: usize;
        for (j, char) in line.iter().enumerate() {
            if !current_number.is_empty() {
                if char.is_numeric() {
                    current_number.push(*char);
                    if j == h_size - 1 {
                        end_index = j;
                        if is_symbol_nearby(&text, i, start_index, end_index, None) {
                            let selected_number = line
                                .get(start_index..=end_index)
                                .unwrap()
                                .iter()
                                .collect::<String>();
                            total += selected_number.parse::<u64>().unwrap();
                        }
                        current_number = String::new();
                    }
                } else {
                    end_index = j - 1;
                    if is_symbol_nearby(&text, i, start_index, end_index, None) {
                        let selected_number = line
                            .get(start_index..=end_index)
                            .unwrap()
                            .iter()
                            .collect::<String>();
                        total += selected_number.parse::<u64>().unwrap();
                    }
                    current_number = String::new();
                }
            } else if char.is_numeric() {
                current_number.push(*char);
                start_index = j;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let text: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let h_size = text[0].len();
    let mut stars: HashMap<(usize, usize), Vec<u64>> = HashMap::new();
    for (i, line) in text.iter().enumerate() {
        let mut current_number: String = String::new();
        let mut start_index: usize = 0;
        let mut end_index: usize;
        for (j, char) in line.iter().enumerate() {
            if !current_number.is_empty() {
                if char.is_numeric() {
                    current_number.push(*char);
                    if j == h_size - 1 {
                        end_index = j;
                        if is_symbol_nearby(&text, i, start_index, end_index, Some(&mut stars)) {
                            let _selected_number = line
                                .get(start_index..=end_index)
                                .unwrap()
                                .iter()
                                .collect::<String>();
                        }
                        current_number = String::new();
                    }
                } else {
                    end_index = j - 1;
                    if is_symbol_nearby(&text, i, start_index, end_index, Some(&mut stars)) {
                        let _selected_number = line
                            .get(start_index..=end_index)
                            .unwrap()
                            .iter()
                            .collect::<String>();
                    }
                    current_number = String::new();
                }
            } else if char.is_numeric() {
                current_number.push(*char);
                start_index = j;
            }
        }
    }

    let mut total = 0;
    for (_, numbers) in stars {
        if numbers.len() == 2 {
            total += numbers[0] * numbers[1];
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
