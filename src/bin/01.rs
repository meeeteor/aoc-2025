use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"([LR])(\d+)").unwrap();
    let mut state: i32 = 50;
    let mut counter: u64 = 0;
    for (_, [dir, count]) in re.captures_iter(input).map(|c| c.extract()) {
        let mul: i32 = if dir == "L" { -1 } else { 1 };
        let count: i32 = count.parse().unwrap();

        state = (state + mul * count) % 100;
        if state == 0 {
            counter += 1
        }
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"([LR])(\d+)").unwrap();
    let mut state: i32 = 50;
    let mut counter = 0;
    for (_, [dir, count]) in re.captures_iter(input).map(|c| c.extract()) {
        let delta: i32 = count.parse().unwrap();

        if dir == "L" {
            // reverse to fix the annoying negatives
            let reverse = (100 - state) % 100;
            counter += (reverse + delta) / 100;
            state = (state - delta).rem_euclid(100);
        } else if dir == "R" {
            counter += (state + delta) / 100;
            state = (state + delta).rem_euclid(100);
        }
    }

    Some(counter as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
