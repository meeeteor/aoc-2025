use std::str::Lines;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let (values, ranges) = get_ranges_and_values(&mut lines);


    let mut count = 0;
    for value in values {
        let in_range = ranges.binary_search_by(|&(start,end)| {
            if value < start { std::cmp::Ordering::Greater }
            else if value > end { std::cmp::Ordering::Less }
            else { std::cmp::Ordering::Equal }
        }).is_ok();
        if in_range { count += 1; }
    }

    Some(count)
}

fn get_ranges_and_values(lines: &mut Lines) -> (Vec<u64>, Vec<(u64, u64)>) {
    let mut ranges: Vec<(u64, u64)> = Vec::new();

    for line in lines.by_ref() {
        if line.is_empty() { break; }
        let (a, b) = line.split_once('-').unwrap();
        ranges.push((a.parse().unwrap(), b.parse().unwrap()));
    }

    let values: Vec<u64> = lines.map(|line| line.parse().unwrap()).collect();

    // merge ranges
    ranges.sort_unstable_by_key(|&(start, _)| start);

    let mut final_ranges: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
    let mut current = ranges[0];

    for &(start, end) in &ranges[1..] {
        if start <= current.1 + 1 {
            current.1 = current.1.max(end)
        } else {
            final_ranges.push(current);
            current = (start, end);
        }
    }
    final_ranges.push(current);
    (values, final_ranges)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let (_, ranges) = get_ranges_and_values(&mut lines);
    
    let mut count = 0;
    for &(start, end) in &ranges {
        count += end - start + 1;
    }
    
    
    Some(count)
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
        assert_eq!(result, Some(14));
    }
}
