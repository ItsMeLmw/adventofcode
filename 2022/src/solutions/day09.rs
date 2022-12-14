// https://adventofcode.com/2022/day/9

use std::collections::HashSet;

fn parse(input: &str) -> Vec<(char, usize)> {
    return input
        .split("\n")
        .map(|line| {
            let mut iter = line.chars();
            let dir = iter.next().unwrap();
            let num = iter.skip(1).collect::<String>().parse::<usize>().unwrap();
            (dir, num)
        })
        .collect::<Vec<(char, usize)>>();
}

fn update_tail(head: (isize, isize), tail: &mut (isize, isize)) -> bool {
    if head.0.abs_diff(tail.0) == 2 || head.1.abs_diff(tail.1) == 2 {
        tail.0 += head.0.cmp(&tail.0) as isize;
        tail.1 += head.1.cmp(&tail.1) as isize;
        return true;
    }
    return false;
}

fn do_move(knots: &mut Vec<(isize, isize)>, dir: char) {
    match dir {
        'L' => knots[0].0 -= 1,
        'R' => knots[0].0 += 1,
        'U' => knots[0].1 -= 1,
        'D' => knots[0].1 += 1,
        _ => unreachable!(),
    }
    for i in 1..knots.len() {
        if !update_tail(knots[i - 1], &mut knots[i]) {
            break;
        }
    }
}

fn unique_tail_locs(moves: &Vec<(char, usize)>, length: usize) -> usize {
    let mut knots = vec![(0, 0); length];
    let mut locs = HashSet::<(isize, isize)>::new();
    for (dir, num) in moves {
        for _ in 0..*num {
            do_move(&mut knots, *dir);
            locs.insert(*knots.last().unwrap());
        }
    }
    return locs.len();
}

fn solve1(parsed: &Vec<(char, usize)>) -> String {
    return unique_tail_locs(parsed, 2).to_string();
}

fn solve2(parsed: &Vec<(char, usize)>) -> String {
    return unique_tail_locs(parsed, 10).to_string();
}

pub fn solve(input: &str) -> (String, String) {
    let parsed = parse(input);
    return (solve1(&parsed), solve2(&parsed));
}
