use std::collections::HashMap;
use regex::{Match, Regex};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let re1 = Regex::new(r"\d+").unwrap();
    let re2 = Regex::new(r"[\+\*]").unwrap();

    let mut digit_map: HashMap<usize, Vec<u64>> = HashMap::new();
    let mut operations: Vec<&str> = Vec::new();

    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        if lines.peek().is_some() {
            re1.find_iter(line).enumerate().for_each(|(i, line)| {
                digit_map.entry(i).or_insert(Vec::new()).push(line.as_str().parse::<u64>().unwrap())
            });
        } else {
            operations = re2.find_iter(line).map(|line| line.as_str()).collect();
        }
    }

    let res = digit_map.into_iter().fold(0, |acc, (i, vec)| {
        acc + match operations[i] {
            "*" => {
                vec.into_iter().fold(1, |acc, x| acc * x)
            }
            "+" => {
                vec.into_iter().fold(0, |acc, x| acc + x)
            }
            &_ => unreachable!(),
        }
    });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    // Operators:
    let re_op = Regex::new(r"[\+\*]\s*").unwrap();
    let mut operators: Vec<(&str, usize)> = Vec::new();
    let op_line = lines.next_back().unwrap();

    re_op.find_iter(op_line).for_each(|mtch: Match| {
        operators.push((mtch.as_str().trim_end(), mtch.len()-1));
    });

    operators.last_mut().unwrap().1 += 1;

    // Values:
    let mut values: Vec<Vec<String>> = vec![vec![];operators.len()];

    // let mut factor = 1;

    // For each line:
    while let Some(line) = lines.next() {
        let mut chars = line.chars();
        operators.iter().enumerate().for_each(|(word_idx, (_, word_length))| {
            values[word_idx].resize(*word_length, "".to_string());

            chars.by_ref().take(*word_length).enumerate().for_each(|(char_idx, char)| {
                if char.is_ascii_digit() {
                    values[word_idx][char_idx].push(char);
                }
            });
            chars.by_ref().next(); // skip the extra space
        });
    }

    let mut total = 0;
    for i in 0..operators.len() {
        match operators[i].0 {
            "*" => {
                total += values[i].iter().fold(1, |acc, x| acc * x.parse::<u64>().unwrap());
            },
            "+" => {
                total += values[i].iter().fold(0, |acc, x| acc + x.parse::<u64>().unwrap());
            },
            _ => {},
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
