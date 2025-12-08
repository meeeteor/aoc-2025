use std::collections::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut beams: HashSet<usize> = HashSet::new();

    if let Some(start) = lines.next() {
        if let Some(pos) = start.find('S') {
            beams.insert(pos);
        }
    }

    let mut split_count = 0;
    while let Some(line) = lines.next() {
        for beam in &beams.clone() {
            let char = line.chars().collect_vec()[*beam];
            if char == '^' {
                beams.insert(beam-1);
                beams.insert(beam+1);
                beams.remove(&beam);
                split_count += 1;
            }
        }
    }

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut row: usize = 0;
    let mut cols: HashMap<usize, usize> = HashMap::new(); // column -> number of lines

    let first_line = lines.next().unwrap();
    let c = first_line.find('S').unwrap();
    cols.insert(c, 1);

    // println!("Row {row}: {first_line}\t{cols:?}");

    while let Some(line) = lines.next() {
        row += 1;
        let splitters = line.chars().positions(|c|c == '^').collect_vec();
        
        for (col, count) in &cols.clone() {
            for splitter in &splitters {
                if col == splitter {
                    cols.remove(col);
                    *cols.entry(col-1).or_insert(0) += count;
                    *cols.entry(col+1).or_insert(0) += count;
                    break;
                }
            }
        }
        
        // println!("Row {row}: {line}\t{cols:?}");
    }
    
    let total: usize = cols.values().sum();
    
    Some(total as u64)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
