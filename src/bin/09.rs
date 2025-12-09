use std::collections::{HashSet};
use itertools::{Either, Itertools};
use rayon::prelude::*;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    input.lines()
        .map(|line: &str|
                 line.split(',')
                     .map(|v|v.parse::<u64>().unwrap())
                     .collect_tuple::<(_, _)>().unwrap())
        .tuple_combinations::<(_,_)>()
        .map(|((x1,y1),(x2,y2)): ((u64,u64),(u64,u64))| (x1.abs_diff(x2)+1) * (y1.abs_diff(y2)+1))
        .max()
}


struct Polygon {
    vertices: Vec<(u64, u64)>,
    horizontal_edges: Vec<((u64, u64), u64)>,   // ((minx, maxx), y)
    vertical_edges: Vec<(u64, (u64, u64))>,     // (x, (miny, maxy))
}

pub fn part_two(input: &str) -> Option<u64> {
    // get candidates; sort on area size;
    let vertices: Vec<(u64, u64)> = input.lines()
        .map(|line: &str| line.split(',')
            .map(|v: &str| v.parse::<u64>().unwrap())
            .collect_tuple::<(_,_)>().unwrap())
        .collect_vec();

    let polygon = build_polygon(&vertices);

    let rect_iterator = vertices.iter().copied()
        .tuple_combinations::<(_,_)>()
        .map(|(p1, p2): ((u64, u64), (u64, u64))| {
            let (min_x, max_x): (u64, u64) = (p1.0.min(p2.0), p1.0.max(p2.0));
            let (min_y, max_y): (u64, u64) = (p1.1.min(p2.1), p1.1.max(p2.1));
            let area = (max_x - min_x + 1) * (max_y - min_y + 1);
            (area, (min_x, min_y), (max_x, max_y))
        })
        .sorted_by_key(|(a,_,_)| *a)
        .rev();

    for (area, p1, p2) in rect_iterator {
        // print!("Checking: {area}:\t{p1:?}\t{p2:?}\t");
        if rectangle_in_polygon(p1, p2, &polygon) {
            return Some(area);
        }
    }

    None
}

// assumes point1 is topleft, point2 is bottom-right
fn rectangle_in_polygon(point1: (u64, u64), point2: (u64,u64), polygon: &Polygon) -> bool {
    // First, check all the corners
    for point in [(point1.0, point1.1), (point1.0, point2.1), (point2.0, point1.1), (point2.0, point2.1)] {
        if !point_in_polygon(point, polygon) {
            // println!("Rejected for corner {point:?}");
            return false;
        }
    }

    // Second, check that none of the edges cross any polygon edge
    for &((x1, x2), y) in &polygon.horizontal_edges {
        if point1.1 < y && y < point2.1 {  // The vertical distance of the polygon is both above and below the edge we are checking
            if x1 < point2.0 && x2 > point1.0 {
                // println!("Rejected on horizontal edge");
                return false;
            }
        }
    }

    for &(x, (y1, y2)) in &polygon.vertical_edges {
        if point1.0 < x && x < point2.0 {  // The vertical distance of the polygon is both above and below the edge we are checking
            if y1 < point2.1 && y2 > point1.1 {
                // println!("Rejected on vertical edge");
                return false;
            }
        }
    }

    // Third, check if none of the vertices are INSIDE the rectangle
    for &(x, y) in &polygon.vertices {
        if point1.0 < x && x < point2.1
            && point1.1 < y && y < point2.1 {
            // this vertex is INSIDE the rectangle!
            // println!("Rejected on vertex {x:?}, {y:?} being inside");
            return false;
        }
    }

    // println!("Accepted");

    true
}

fn point_in_polygon(point: (u64, u64), polygon: &Polygon) -> bool {
    if polygon.vertices.par_iter().any(|(x, y)| point.0 == *x && point.1 == *y) {
        return true;
    }

    let n_crossings = polygon.vertical_edges.iter().copied().filter(|(x, (y1, y2))| {
        return point.1 >= *y1
            && point.1 < *y2
            && point.0 <= *x
    }).count();

    n_crossings % 2 == 1
}

fn build_polygon(vertices: &Vec<(u64, u64)>) -> Polygon {
    let (vertical_edges, horizontal_edges): (Vec<(u64, (u64, u64))>, Vec<((u64, u64), u64)>) = vertices.iter()
        .circular_tuple_windows::<(_,_)>()
        .partition_map(|(p1, p2)| {
            if p1.0 == p2.0 {
                // vertical
                Either::Left((p1.0, (p1.1.min(p2.1), p1.1.max(p2.1))))
            } else {
                // horizontal
                Either::Right(((p1.0.min(p2.0), p1.0.max(p2.0)), p1.1))
            }
        });

    Polygon{
        vertices: vertices.clone(),
        horizontal_edges,
        vertical_edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(25));
    }
}
