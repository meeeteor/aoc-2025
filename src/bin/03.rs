advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input.lines().fold(0u32, |acc, line| {
        let (l, r) = line.bytes().rfold((0,0), |(l,r), b| {
            let digit = (b - b'0') as u32;

            // First candidate is simply the two right-most digits
            if r == 0 { return (digit, l); }

            // If the digit is bigger than left, it is always worth more
            if digit > l {
                return (digit, if l > r { l } else { r });
            }

            // If the digit is equal to left, it may be worth it if right is less:
            if digit == l && digit > r { return (digit, l) }

            // In any other case, ignore digit
            (l, r)
        });
        return acc + l*10+r;
    });

    Some(sum as u64)
}

const POWERS: [u64; 12] = [
    1, 10, 100,
    1_000, 10_000, 100_000,
    1_000_000, 10_000_000, 100_000_000,
    1_000_000_000, 10_000_000_000, 100_000_000_000,
];

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0u64;

    for line in input.lines() {
        let mut digits = [0u32; 12];
        let mut len = 0usize;

        for byte in line.bytes().rev() {
            let digit = (byte - b'0') as u32;

            // First, get a first solution
            if len < 12 {
                digits[len] = digit;
                len += 1;
                continue;
            }

            // For each next solution, check from left to right:
            // If the digit is larger: append, continue for the replaced digit instead.
            let mut checking = digit;
            for index in (0..12).rev() {
                let current = digits[index];

                if checking <= current {
                    if checking < current {
                        break;
                    }
                    // checking == current, continue
                    continue;
                }

                // checking > current
                std::mem::swap(&mut checking, &mut digits[index]);
            }
        }

        // Unroll or use array indexing directly
        for i in 0..12 {
            sum += (digits[i] as u64) * POWERS[i];
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
