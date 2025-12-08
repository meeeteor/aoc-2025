use itertools::Itertools;

advent_of_code::solution!(8);


#[derive(Clone)]
#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
    network: Option<usize>,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_h(input, 1000)
}
pub fn part_one_h(input: &str, n_iterations: usize) -> Option<u64> {
    let mut points = parse_input(input);

    let mut pairs: Vec<_> = (0..points.len()).tuple_combinations::<(_, _)>().collect();

    pairs.sort_by(|(i1, j1), (i2, j2)| {
        let d1 = points[*i1].distance(&points[*j1]);
        let d2 = points[*i2].distance(&points[*j2]);
        d1.partial_cmp(&d2).unwrap()
    });

    let mut next_network_id = 1;
    for i in 0..n_iterations {
        connect_pair(i, &pairs, &mut points, &mut next_network_id);
    }

    let mut networks: Vec<usize> = points.into_iter().filter(|p| p.network.is_some()).into_group_map_by(|p| p.network).into_iter().map(|(_, v)| v.len()).collect();

    networks.sort();
    networks.reverse();

    Some((networks[0] * networks[1] * networks[2]) as u64)
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut points = parse_input(input);
    let mut pairs: Vec<_> = (0..points.len()).tuple_combinations::<(_, _)>().collect();

    pairs.sort_by(|(i1, j1), (i2, j2)| {
        let d1 = points[*i1].distance(&points[*j1]);
        let d2 = points[*i2].distance(&points[*j2]);
        d1.partial_cmp(&d2).unwrap()
    });

    let mut next_network_id = 1;
    let mut idx = 0;
    loop {
        connect_pair(idx, &pairs, &mut points, &mut next_network_id);
        if points.iter().all(|p| p.network.is_some() && p.network == points[0].network) {
            break;
        }
        idx += 1;
    }

    let (i1, i2) = pairs[idx];

    Some((points[i1].x * points[i2].x) as u64)
}



fn connect_pair(
    idx: usize,
    pairs: &Vec<(usize, usize)>,
    points: &mut [Point],
    nxt_nw: &mut usize) {
    let (i1, i2): &(usize, usize) = &pairs[idx];

    if points[*i1].network.is_some() && points[*i2].network.is_some() {
        // combine networks to be points[*i1]
        let id1 = points[*i1].network;
        let id2 = points[*i2].network;

        points.iter_mut().filter(|p| p.network == id2).for_each(|p| {
            p.network = id1;
        });
        return;
    }
    if points[*i1].network.is_some() && points[*i2].network.is_none() {
        // add points[*i2] to points[*i1] network
        points[*i2].network = points[*i1].network;
        return;
    }
    if points[*i1].network.is_none() && points[*i2].network.is_some() {
        // add points[*i1] to points[*i2] network
        points[*i1].network = points[*i2].network;
        return;
    }
    // create a new network;
    points[*i1].network = Some(*nxt_nw);
    points[*i2].network = Some(*nxt_nw);
    *nxt_nw += 1;
}

fn parse_input(input: &str) -> Vec<Point> {
    let points: Vec<Point> = input.lines().map(|line| {
        let values: Vec<_> = line.split(',').map(|s| s.parse::<isize>().unwrap()).collect();
        Point { x: values[0], y: values[1], z: values[2], network: None }
    }).collect();
    points
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_h(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
