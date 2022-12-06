use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Move {
    quanity: usize,
    src: usize,
    dest: usize,
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        if let Some(caps) = RE.captures(s) {
            Ok(Move {
                quanity: caps[1].parse().unwrap(),
                src: caps[2].parse::<usize>().unwrap() - 1,
                dest: caps[3].parse::<usize>().unwrap() - 1,
            })
        } else {
            Err(Error::msg("no match"))
        }
    }
}

impl Move {
    fn exec(&self, stacks: &mut Vec<Vec<char>>) {
        for _ in 0..self.quanity {
            let ch = stacks[self.src].pop().unwrap();
            stacks[self.dest].push(ch);
        }
    }

    fn exec_2(&self, stacks: &mut [Vec<char>]) {
        let (left, right) = stacks.split_at_mut(self.src.max(self.dest));
        let (src, dest): (&mut Vec<char>, &mut Vec<char>) = if self.src > self.dest {
            (&mut right[0], &mut left[self.dest])
        } else {
            (&mut left[self.src], &mut right[0])
        };

        let pos = src.len() - self.quanity;
        dest.extend_from_slice(&src[pos..]);
        src.truncate(pos);
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let parts: Vec<_> = input.split("\n\n").collect();
    let stacks = parts[0]
        .lines()
        .map(|l| {
            l.chars()
                .chunks(4)
                .into_iter()
                .map(|chs| {
                    chs.skip(1).next().and_then(|ch| {
                        if ch == ' ' {
                            None
                        } else if ch.is_digit(10) {
                            None
                        } else {
                            Some(ch)
                        }
                    })
                })
                .collect::<Vec<Option<char>>>()
        })
        .fold(Vec::new(), |mut acc, row| {
            row.iter().enumerate().for_each(|(i, ch)| {
                if let Some(c) = ch {
                    while acc.len() < i + 1 {
                        acc.push(Vec::new());
                    }
                    acc[i].insert(0, *c);
                }
            });
            acc
        });

    let moves = parts[1].lines().map(|l| l.parse().unwrap()).collect();

    (stacks, moves)
}

fn tops(stacks: &Vec<Vec<char>>) -> String {
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn main() -> Result<()> {
    part_1();
    part_2();
    Ok(())
}

fn part_1() {
    let (mut diagram, moves) = parse(INPUT);

    for m in moves {
        m.exec(&mut diagram);
    }

    println!("Part 1: {}", tops(&diagram));
}

fn part_2() {
    let (mut diagram, moves) = parse(INPUT);

    for m in moves {
        m.exec_2(&mut diagram);
    }

    println!("Part 2: {}", tops(&diagram));
}

const INPUT: &str = r#"        [H]         [S]         [D]
    [S] [C]         [C]     [Q] [L]
    [C] [R] [Z]     [R]     [H] [Z]
    [G] [N] [H] [S] [B]     [R] [F]
[D] [T] [Q] [F] [Q] [Z]     [Z] [N]
[Z] [W] [F] [N] [F] [W] [J] [V] [G]
[T] [R] [B] [C] [L] [P] [F] [L] [H]
[H] [Q] [P] [L] [G] [V] [Z] [D] [B]
 1   2   3   4   5   6   7   8   9

move 2 from 7 to 2
move 1 from 4 to 8
move 2 from 1 to 9
move 4 from 6 to 5
move 1 from 7 to 6
move 2 from 1 to 4
move 7 from 8 to 9
move 7 from 4 to 5
move 4 from 2 to 4
move 1 from 5 to 9
move 14 from 5 to 4
move 1 from 3 to 8
move 5 from 4 to 8
move 1 from 2 to 5
move 2 from 4 to 1
move 6 from 8 to 1
move 1 from 8 to 6
move 1 from 2 to 5
move 5 from 3 to 7
move 2 from 6 to 3
move 2 from 4 to 7
move 3 from 3 to 9
move 7 from 4 to 1
move 1 from 6 to 9
move 2 from 6 to 1
move 3 from 5 to 2
move 1 from 1 to 8
move 21 from 9 to 1
move 1 from 4 to 2
move 7 from 7 to 2
move 1 from 4 to 2
move 23 from 1 to 5
move 5 from 5 to 1
move 1 from 3 to 6
move 1 from 6 to 3
move 12 from 1 to 6
move 1 from 3 to 6
move 2 from 1 to 8
move 1 from 9 to 3
move 2 from 8 to 1
move 2 from 1 to 8
move 1 from 1 to 3
move 2 from 3 to 1
move 2 from 8 to 1
move 3 from 6 to 1
move 1 from 8 to 7
move 4 from 6 to 2
move 3 from 6 to 9
move 2 from 5 to 7
move 2 from 7 to 8
move 1 from 7 to 9
move 9 from 1 to 5
move 12 from 5 to 9
move 1 from 8 to 6
move 1 from 6 to 9
move 1 from 6 to 9
move 7 from 9 to 4
move 10 from 2 to 1
move 12 from 5 to 4
move 7 from 4 to 9
move 7 from 4 to 7
move 1 from 5 to 4
move 7 from 7 to 8
move 1 from 6 to 3
move 1 from 3 to 1
move 3 from 2 to 4
move 1 from 6 to 8
move 7 from 1 to 2
move 1 from 6 to 7
move 12 from 9 to 4
move 3 from 8 to 5
move 1 from 7 to 3
move 6 from 9 to 1
move 10 from 1 to 9
move 7 from 9 to 5
move 3 from 9 to 5
move 1 from 3 to 4
move 2 from 2 to 1
move 1 from 5 to 1
move 9 from 4 to 3
move 1 from 1 to 3
move 8 from 4 to 7
move 7 from 5 to 3
move 2 from 7 to 2
move 8 from 3 to 9
move 1 from 1 to 8
move 10 from 2 to 3
move 4 from 8 to 7
move 12 from 3 to 4
move 9 from 7 to 2
move 2 from 1 to 3
move 1 from 9 to 6
move 2 from 4 to 9
move 1 from 7 to 6
move 5 from 5 to 9
move 8 from 3 to 1
move 2 from 6 to 3
move 14 from 4 to 3
move 15 from 3 to 9
move 1 from 3 to 1
move 3 from 9 to 8
move 1 from 8 to 1
move 1 from 3 to 2
move 5 from 2 to 8
move 1 from 4 to 2
move 2 from 1 to 3
move 2 from 3 to 9
move 3 from 2 to 4
move 6 from 1 to 8
move 2 from 2 to 6
move 1 from 6 to 4
move 2 from 4 to 7
move 5 from 8 to 5
move 1 from 6 to 9
move 7 from 9 to 6
move 1 from 5 to 3
move 2 from 7 to 8
move 2 from 2 to 4
move 3 from 5 to 6
move 1 from 3 to 8
move 1 from 5 to 6
move 2 from 4 to 1
move 3 from 1 to 6
move 21 from 9 to 5
move 1 from 4 to 3
move 1 from 4 to 9
move 2 from 9 to 2
move 1 from 3 to 9
move 4 from 2 to 3
move 3 from 8 to 1
move 14 from 5 to 9
move 7 from 5 to 4
move 3 from 8 to 4
move 4 from 3 to 2
move 3 from 8 to 5
move 1 from 2 to 3
move 1 from 5 to 1
move 2 from 5 to 4
move 3 from 2 to 9
move 11 from 4 to 1
move 17 from 9 to 2
move 17 from 2 to 9
move 10 from 9 to 2
move 2 from 8 to 2
move 3 from 8 to 3
move 8 from 9 to 7
move 4 from 7 to 3
move 2 from 3 to 2
move 3 from 2 to 3
move 9 from 3 to 5
move 1 from 1 to 9
move 8 from 5 to 1
move 2 from 7 to 9
move 24 from 1 to 3
move 24 from 3 to 6
move 1 from 5 to 3
move 10 from 2 to 1
move 1 from 4 to 5
move 3 from 9 to 1
move 1 from 3 to 5
move 17 from 6 to 5
move 1 from 7 to 4
move 13 from 5 to 4
move 3 from 5 to 8
move 1 from 7 to 9
move 3 from 6 to 9
move 8 from 6 to 4
move 1 from 9 to 6
move 11 from 1 to 8
move 1 from 5 to 6
move 12 from 4 to 9
move 2 from 5 to 1
move 1 from 1 to 7
move 5 from 9 to 2
move 1 from 7 to 9
move 3 from 1 to 5
move 3 from 5 to 9
move 7 from 9 to 3
move 4 from 9 to 6
move 3 from 6 to 8
move 5 from 4 to 3
move 2 from 2 to 6
move 3 from 9 to 3
move 3 from 6 to 4
move 4 from 2 to 6
move 11 from 3 to 5
move 11 from 6 to 9
move 2 from 3 to 5
move 1 from 5 to 8
move 3 from 6 to 2
move 7 from 9 to 2
move 8 from 5 to 7
move 6 from 4 to 5
move 2 from 4 to 3
move 1 from 8 to 6
move 4 from 8 to 3
move 13 from 8 to 3
move 1 from 9 to 5
move 6 from 7 to 2
move 1 from 7 to 6
move 1 from 6 to 5
move 2 from 6 to 7
move 13 from 3 to 5
move 6 from 2 to 7
move 1 from 6 to 1
move 1 from 2 to 8
move 2 from 7 to 8
move 14 from 5 to 8
move 1 from 1 to 4
move 9 from 2 to 1
move 14 from 8 to 7
move 3 from 3 to 9
move 11 from 5 to 3
move 1 from 4 to 5
move 4 from 9 to 8
move 4 from 8 to 7
move 5 from 3 to 9
move 11 from 7 to 8
move 9 from 1 to 3
move 4 from 3 to 2
move 6 from 8 to 4
move 2 from 8 to 2
move 13 from 3 to 6
move 1 from 4 to 1
move 5 from 4 to 2
move 10 from 2 to 6
move 4 from 9 to 1
move 8 from 7 to 8
move 10 from 8 to 5
move 2 from 3 to 2
move 2 from 8 to 6
move 1 from 7 to 1
move 2 from 7 to 6
move 2 from 2 to 9
move 2 from 8 to 6
move 6 from 1 to 7
move 5 from 9 to 1
move 4 from 7 to 8
move 1 from 7 to 2
move 2 from 1 to 7
move 1 from 3 to 8
move 1 from 1 to 6
move 2 from 2 to 6
move 1 from 7 to 8
move 1 from 1 to 9
move 8 from 5 to 7
move 2 from 7 to 9
move 9 from 6 to 3
move 13 from 6 to 8
move 3 from 9 to 1
move 5 from 6 to 1
move 3 from 8 to 1
move 3 from 3 to 4
move 1 from 4 to 3
move 1 from 4 to 8
move 4 from 6 to 3
move 11 from 8 to 2
move 1 from 6 to 9
move 8 from 3 to 9
move 3 from 5 to 8
move 4 from 1 to 2
move 6 from 8 to 5
move 6 from 5 to 1
move 5 from 1 to 3
move 3 from 3 to 4
move 3 from 8 to 4
move 2 from 4 to 5
move 10 from 7 to 8
move 5 from 9 to 2
move 1 from 7 to 5
move 3 from 5 to 2
move 4 from 9 to 3
move 4 from 1 to 5
move 1 from 3 to 2
move 3 from 5 to 2
move 6 from 2 to 5
move 10 from 8 to 3
move 4 from 4 to 5
move 4 from 2 to 8
move 12 from 3 to 8
move 1 from 1 to 3
move 9 from 8 to 6
move 1 from 4 to 1
move 6 from 8 to 7
move 3 from 1 to 7
move 9 from 5 to 7
move 11 from 7 to 2
move 2 from 7 to 3
move 9 from 2 to 7
move 1 from 8 to 7
move 1 from 5 to 2
move 2 from 6 to 2
move 2 from 1 to 2
move 6 from 3 to 5
move 2 from 3 to 6
move 4 from 7 to 3
move 3 from 3 to 1
move 2 from 1 to 5
move 7 from 7 to 6
move 1 from 1 to 5
move 3 from 2 to 4
move 1 from 3 to 2
move 18 from 2 to 1
move 4 from 2 to 7
move 6 from 5 to 9
move 1 from 4 to 8
move 2 from 6 to 1
move 19 from 1 to 2
move 4 from 9 to 5
move 5 from 7 to 2
move 1 from 8 to 7
move 1 from 1 to 2
move 6 from 5 to 7
move 1 from 3 to 8
move 6 from 7 to 6
move 1 from 4 to 1
move 4 from 7 to 9
move 1 from 1 to 3
move 1 from 2 to 5
move 1 from 4 to 8
move 1 from 3 to 4
move 3 from 5 to 4
move 2 from 8 to 9
move 9 from 2 to 4
move 19 from 6 to 4
move 1 from 4 to 7
move 5 from 9 to 5
move 10 from 2 to 9
move 2 from 5 to 4
move 14 from 4 to 7
move 2 from 2 to 1
move 3 from 9 to 1
move 1 from 1 to 3
move 13 from 7 to 6
move 1 from 5 to 9
move 1 from 6 to 9
move 1 from 7 to 2
move 5 from 9 to 7
move 1 from 5 to 2
move 3 from 7 to 3
move 3 from 4 to 9
move 1 from 5 to 2
move 4 from 4 to 2
move 2 from 7 to 3
move 4 from 1 to 6
move 1 from 7 to 9
move 11 from 9 to 5
move 8 from 2 to 9
move 6 from 9 to 6
move 8 from 4 to 5
move 14 from 5 to 6
move 1 from 5 to 4
move 3 from 5 to 1
move 1 from 5 to 2
move 2 from 6 to 4
move 2 from 4 to 2
move 1 from 9 to 2
move 1 from 2 to 3
move 1 from 9 to 3
move 3 from 2 to 7
move 7 from 6 to 7
move 5 from 4 to 3
move 23 from 6 to 1
move 5 from 7 to 2
move 22 from 1 to 6
move 6 from 6 to 3
move 6 from 2 to 4
move 6 from 4 to 1
move 3 from 7 to 8
move 3 from 1 to 8
move 4 from 3 to 2
move 1 from 1 to 3
move 3 from 3 to 1
move 1 from 7 to 5
move 1 from 6 to 5
move 1 from 7 to 4
move 4 from 6 to 9
move 5 from 3 to 6
move 2 from 2 to 1
move 3 from 9 to 4
move 11 from 1 to 9
move 2 from 4 to 7
move 4 from 6 to 1
move 1 from 5 to 4
move 5 from 8 to 9
move 1 from 7 to 1
move 3 from 2 to 7
move 4 from 1 to 2
move 3 from 4 to 2
move 1 from 8 to 5
move 1 from 5 to 4
move 1 from 5 to 4
move 5 from 6 to 1
move 3 from 7 to 6
move 5 from 2 to 8
move 15 from 9 to 2
move 1 from 3 to 9
move 10 from 6 to 8
move 1 from 4 to 9
move 1 from 8 to 3
move 1 from 4 to 6
move 4 from 6 to 3
move 2 from 9 to 7
move 1 from 7 to 6
move 1 from 1 to 6
move 3 from 3 to 8
move 2 from 7 to 8
move 3 from 8 to 4
move 12 from 2 to 9
move 14 from 9 to 5
move 12 from 8 to 2
move 1 from 6 to 7
move 8 from 3 to 1
move 2 from 4 to 6
move 1 from 3 to 6
move 5 from 6 to 1
move 17 from 1 to 2
move 29 from 2 to 1
move 1 from 8 to 5
move 1 from 4 to 3
move 1 from 8 to 5
move 1 from 8 to 7
move 5 from 2 to 1
move 1 from 3 to 5
move 1 from 6 to 4
move 6 from 5 to 8
move 1 from 4 to 9
move 1 from 7 to 2
move 1 from 2 to 6
move 7 from 8 to 7
move 1 from 6 to 9
move 2 from 9 to 2
move 2 from 2 to 8
move 15 from 1 to 2
move 2 from 8 to 3
move 9 from 1 to 2
move 24 from 2 to 7
move 11 from 1 to 2
move 1 from 3 to 1
move 22 from 7 to 6
move 6 from 5 to 2
move 2 from 6 to 5
move 1 from 1 to 9
move 1 from 9 to 6
move 6 from 5 to 1
move 12 from 6 to 2
move 3 from 1 to 5
move 1 from 3 to 2
move 25 from 2 to 6
move 4 from 7 to 5
move 8 from 5 to 4
move 4 from 4 to 8
move 1 from 1 to 8
move 5 from 8 to 4
move 4 from 4 to 1
move 2 from 1 to 9
move 20 from 6 to 8
move 4 from 2 to 6
move 19 from 8 to 7
move 2 from 9 to 3
move 1 from 8 to 2
move 11 from 6 to 7
move 3 from 1 to 2
move 5 from 4 to 3
move 1 from 1 to 3
move 1 from 3 to 5
move 2 from 2 to 8
move 33 from 7 to 3
move 1 from 5 to 3
move 1 from 8 to 7
move 1 from 7 to 4
move 5 from 6 to 8
move 2 from 7 to 6
move 2 from 2 to 3
move 1 from 2 to 5
move 1 from 7 to 9
move 1 from 5 to 7
move 1 from 8 to 2
move 1 from 4 to 3
move 43 from 3 to 7
move 1 from 3 to 8
move 1 from 6 to 8
move 8 from 7 to 5
move 3 from 5 to 3
move 1 from 6 to 4
move 2 from 6 to 7
move 4 from 8 to 7
move 3 from 3 to 2
move 1 from 9 to 6
move 3 from 8 to 3
move 1 from 6 to 7
move 1 from 4 to 6
move 1 from 3 to 7
move 1 from 3 to 2
move 5 from 2 to 5
move 1 from 6 to 1
move 1 from 3 to 2
move 42 from 7 to 5
move 44 from 5 to 4
move 2 from 5 to 8
move 1 from 7 to 3
move 16 from 4 to 6
move 3 from 5 to 9"#;
