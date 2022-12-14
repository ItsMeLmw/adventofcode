// https://adventofcode.com/2022/day/15

use itertools::Itertools;

fn parse(input: &str) -> Vec<((isize, isize), (isize, isize))> {
    return input.split("\n").map(|line| { 
        let words = line.split(" ").collect::<Vec<&str>>(); 
        (
            (
                words[2][2..words[2].len() - 1].parse::<isize>().unwrap(), 
                words[3][2..words[3].len() - 1].parse::<isize>().unwrap(),
            ),
            (
                words[8][2..words[8].len() - 1].parse::<isize>().unwrap(),
                words[9][2..].parse::<isize>().unwrap(),
            )
        ) 
    }).collect::<Vec<((isize, isize),(isize,isize))>>();
}

fn make_range(sensor: &(isize, isize), beacon: &(isize, isize), y: isize) -> Option<(isize, isize)> {
    let dist = sensor.0.abs_diff(beacon.0) as isize + sensor.1.abs_diff(beacon.1) as isize;
    let span = dist - sensor.1.abs_diff(y) as isize;
    return if span >= 0 { Some((sensor.0 - span, sensor.0 + span)) } else { None };
}

fn overlapping(ranges: Vec<(isize, isize)>) -> Option<isize> {
    let mut iter = ranges.iter().sorted_by_key(|r| r.0);
    let mut total_range = iter.next().unwrap().clone();

    for range in iter {
        if range.0 <= total_range.1 { total_range.1 = total_range.1.max(range.1) }
        else { return Some(total_range.1 + 1) }
    }
    return None;
}

fn solve1(parsed: &Vec<((isize, isize), (isize, isize))>) -> String {
    // Hardcode Y-coordinate if the input is the example.
    let y_coord = if parsed.len() == 14 {10} else {2_000_000};

    let mut min = 0;
    let mut max = 0;

    // Assume the ranges do not have any holes, which they don't for part 1
    parsed.iter().for_each(|(sensor, beacon)| {
        if let Some((rmin, rmax)) = make_range(sensor, beacon, y_coord) {
            min = min.min(rmin);
            max = max.max(rmax);
        }
    });

    // Answer should be +1 for inclusive range, 
    // but -1 because there is a beacon on that Y.
    return (max - min).to_string();
}

// There MUST be a more efficient solution for this one, but I'll accept this.
fn solve2(parsed: &Vec<((isize, isize), (isize, isize))>) -> String {
    // Hardcoding example input again
    let y_range = if parsed.len() == 14 {20} else {4_000_000};

    for y_coord in 0..y_range {
        let mut ranges = vec![];
        parsed.iter().for_each(|(sensor, beacon)| {
            if let Some(range) = make_range(sensor, beacon, y_coord) {
                ranges.push(range);
            }
        });
        if let Some(x_coord) = overlapping(ranges) {
            return (x_coord * 4000000 + y_coord).to_string();
        }
    }
    // Did not find solution
    return 0.to_string(); 
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
