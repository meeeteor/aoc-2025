use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut total: u64 = 0;
    for (start, end) in re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_,[start,end])| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
    {
        for n in start..=end {
            let str_n: String = n.to_string();
            if str_n.len() % 2 == 1 { continue; }

            let (left, right) = str_n.as_str().split_at(str_n.len() / 2);
            if left == right {
                total += n;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut total: u64 = 0;
    for (start, end) in re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_,[start,end])| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
    {
        for n in start..=end {
            let str_n: String = n.to_string();

            let divisors: Vec<usize> = (1..=str_n.len()/2).filter(|x| str_n.len() % x == 0).collect();

            for divisor in divisors {
                let mut chunks = str_n.as_bytes().chunks_exact(divisor);
                if let Some(fst) = chunks.next() {
                    if chunks.all(|c| c == fst) {
                        total += n;
                        break;
                    }
                }
            }
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
