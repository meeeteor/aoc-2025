use regex::Regex;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut total: u64 = 0;
    for (start, end) in re.captures_iter(input)
        .map(|c| c.extract())
        .map(|(_,[start,end])| (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap()))
    {
        for num in start..=end {
            let n_digits = num.ilog10() + 1;
            if n_digits % 2 == 1 { continue; }

            let divisor = 10u64.pow(n_digits/2);
            if num / divisor == num % divisor {
                total += num;
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
        for num in start..=end {
            let n_digits = num.ilog10() + 1;


            'sizes: for chunk_size in (1..=n_digits/2).filter(|&x|n_digits%x==0) {
                let chunk_count = n_digits / chunk_size;
                let div = 10u64.pow(chunk_size);

                let first = num % div;
                let mut tmp = num / div;
                for _ in 1..chunk_count {
                    if tmp % div != first {
                        continue 'sizes;
                    }
                    tmp /= div;
                }
                total += num;
                break 'sizes;
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
