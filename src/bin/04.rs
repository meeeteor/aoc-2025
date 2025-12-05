advent_of_code::solution!(4);

const NEIGHBOR_VECTORS: [(i32, i32); 8] = [
    (-1,-1),(-1,0),(-1,1),
    (0,-1), (0,1),
    (1,-1), (1,0), (1,1),
];

pub fn part_one(input: &str) -> Option<u64> {
    let storage: Vec<Vec<_>> = input.lines().map(|line: &str| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut count = 0;

    for (i, row) in storage.iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if *cell == '@' && count_neighbors(&storage, i, j) < 4 {
                count += 1;
            }
        }
    }
    Some(count)
}

fn count_neighbors(storage: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> usize {
    let mut count = 0;
    for (offset_row, offset_col) in NEIGHBOR_VECTORS.iter() {
        let row_check = (row_index as i32 + offset_row);
        let col_check = (col_index as i32 + offset_col);

        if row_check < 0 || col_check < 0 { continue; }
        let Some(row) = storage.get(row_check as usize) else { continue; };
        let Some(cell) = row.get(col_check as usize) else { continue; };
        if *cell == '@' { count += 1; };
    }

    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let storage: &mut Vec<Vec<char>> = &mut input.lines().map(|line: &str| {
        line.chars().collect()
    }).collect();

    let mut count = 0;

    loop {
        let mut remove = Vec::new();

        for (i, row) in storage.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell == '@' && count_neighbors(storage, i, j) < 4 {
                    remove.push((i, j));
                }
            }
        }

        if remove.is_empty() {break;}

        for (i, j) in &remove {
            storage[*i][*j] = 'x';
        }

        count += remove.len();
    }

    Some(count as u64)
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
        assert_eq!(result, Some(43));
    }
}
