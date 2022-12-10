use crate::Instruction::{Addx, Noop};
use anyhow::Result;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Noop)
        } else {
            let (op, n) = s.splitn(2, " ").collect_tuple().unwrap();
            assert!(op == "addx");
            Ok(Addx(n.parse().unwrap()))
        }
    }
}

fn run_log(program: &[Instruction]) -> Vec<i32> {
    let mut x = 1;
    let mut log = Vec::new();

    for ins in program {
        match ins {
            Noop => {
                log.push(x);
            }
            Addx(n) => {
                log.push(x);
                log.push(x);
                x += n;
            }
        }
    }

    log.push(x);
    log
}

fn signal_str(log: &[i32], clocks: &[usize]) -> i32 {
    clocks.iter().map(|&t| log[t - 1] * (t as i32)).sum()
}

fn print_image(log: &[i32], width: usize) -> String {
    log.iter()
        .enumerate()
        .map(|(px, &sprite_pos)| {
            if sprite_pos > 0 {
                let pos: usize = sprite_pos.try_into().unwrap();
                if (pos - 1..=pos + 1).contains(&(px % 40)) {
                    '#'
                } else {
                    '.'
                }
            } else {
                '.'
            }
        })
        .chunks(width)
        .into_iter()
        .map(|ss| ss.collect::<String>())
        .join("\n")
}

fn main() -> Result<()> {
    let input: Vec<Instruction> = INPUT.lines().map(|l| l.parse().unwrap()).collect();

    let log = run_log(&input);

    println!(
        "Part 1: {}",
        signal_str(&log, &[20, 60, 100, 140, 180, 220])
    );
    // low 13340

    println!("Part 2:\n{}", print_image(&log, 40));
    Ok(())
}

const INPUT: &str = r#"addx 1
noop
addx 2
addx 11
addx -4
noop
noop
noop
noop
addx 3
addx -3
addx 10
addx 1
noop
addx 12
addx -8
addx 5
noop
noop
addx 1
addx 4
addx -12
noop
addx -25
addx 14
addx -7
noop
addx 11
noop
addx -6
addx 3
noop
addx 2
addx 22
addx -12
addx -17
addx 15
addx 2
addx 10
addx -9
noop
noop
noop
addx 5
addx 2
addx -33
noop
noop
noop
noop
addx 12
addx -9
addx 7
noop
noop
addx 3
addx -2
addx 2
addx 26
addx -31
addx 14
addx 3
noop
addx 13
addx -1
noop
addx -5
addx -13
addx 14
noop
addx -20
addx -15
noop
addx 7
noop
addx 31
noop
addx -26
noop
noop
noop
addx 5
addx 20
addx -11
addx -3
addx 9
addx -5
addx 2
noop
addx 4
noop
addx 4
noop
noop
addx -7
addx -30
noop
addx 7
noop
noop
addx -2
addx -4
addx 11
addx 14
addx -9
addx -2
noop
addx 7
noop
addx -11
addx -5
addx 19
addx 5
addx 2
addx 5
noop
noop
addx -2
addx -27
addx -6
addx 1
noop
noop
addx 4
addx 1
addx 4
addx 5
noop
noop
noop
addx 1
noop
addx 4
addx 1
noop
noop
addx 5
noop
noop
addx 4
addx 1
noop
addx 4
addx 1
noop
noop
noop
noop"#;
