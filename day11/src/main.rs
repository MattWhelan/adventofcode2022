use anyhow::{Error, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Op {
    Add(usize),
    Mul(usize),
    Square,
}

impl Op {
    fn exec(&self, old: usize) -> usize {
        match self {
            Op::Add(n) => old + n,
            Op::Mul(n) => old * n,
            Op::Square => old * old,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<usize>,
    op: Op,
    modulus: usize,
    true_target: usize,
    false_target: usize,
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r#"Monkey (\d+):
  Starting items: ([\d, ]+)
  Operation: new = old (.) (\d+|old)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)"#
            )
            .unwrap();
        }

        if let Some(caps) = RE.captures(s) {
            let items = caps[2]
                .split(", ")
                .map(|item_str| item_str.parse().unwrap())
                .collect();
            let op = if let Ok(n) = caps[4].parse() {
                match &caps[3] {
                    "*" => Op::Mul(n),
                    "+" => Op::Add(n),
                    _ => unreachable!(),
                }
            } else {
                Op::Square
            };

            Ok(Monkey {
                id: caps[1].parse().unwrap(),
                items,
                op,
                modulus: caps[5].parse().unwrap(),
                true_target: caps[6].parse().unwrap(),
                false_target: caps[7].parse().unwrap(),
            })
        } else {
            Err(Error::msg("Didn't match"))
        }
    }
}

#[derive(Debug, Clone)]
struct Forest {
    monkeys: Vec<RefCell<Monkey>>,
    inspection_counts: Vec<usize>,
    very_worried: bool,
    test_modulus: usize,
}

impl Forest {
    fn new(monkeys: &[Monkey], very_worried: bool) -> Forest {
        let test_modulus = monkeys.iter().map(|m| m.modulus).product();

        Forest {
            inspection_counts: vec![0; monkeys.len()],
            monkeys: monkeys
                .into_iter()
                .map(|m| RefCell::new(m.clone()))
                .collect(),
            very_worried,
            test_modulus,
        }
    }

    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            self.turn(i);
        }
    }

    fn turn(&mut self, id: usize) {
        let mut m = self.monkeys[id].borrow_mut();
        let mut t = self.monkeys[m.true_target].borrow_mut();
        let mut f = self.monkeys[m.false_target].borrow_mut();
        for item in m.items.iter() {
            //inspect
            self.inspection_counts[id] += 1;
            let mut new_worry = m.op.exec(*item);
            if self.very_worried {
                new_worry = new_worry % self.test_modulus;
            } else {
                new_worry = new_worry / 3;
            }
            //test
            if new_worry % m.modulus == 0 {
                //throw
                t.items.push(new_worry);
            } else {
                //throw
                f.items.push(new_worry);
            }
        }
        m.items.clear();
    }

    fn monkey_business(&self) -> usize {
        self.inspection_counts
            .iter()
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

fn main() -> Result<()> {
    let input: Vec<Monkey> = INPUT.split("\n\n").map(|l| l.parse().unwrap()).collect();

    {
        let mut forest = Forest::new(&input, false);

        for _ in 0..20 {
            forest.round();
        }

        println!("Part 1: {}", forest.monkey_business());
    }
    {
        let mut forest = Forest::new(&input, true);

        for _ in 0..10000 {
            forest.round();
        }

        println!("Part 2: {}", forest.monkey_business());
    }
    Ok(())
}

const INPUT: &str = r#"Monkey 0:
  Starting items: 57, 58
  Operation: new = old * 19
  Test: divisible by 7
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 66, 52, 59, 79, 94, 73
  Operation: new = old + 1
  Test: divisible by 19
    If true: throw to monkey 4
    If false: throw to monkey 6

Monkey 2:
  Starting items: 80
  Operation: new = old + 6
  Test: divisible by 5
    If true: throw to monkey 7
    If false: throw to monkey 5

Monkey 3:
  Starting items: 82, 81, 68, 66, 71, 83, 75, 97
  Operation: new = old + 5
  Test: divisible by 11
    If true: throw to monkey 5
    If false: throw to monkey 2

Monkey 4:
  Starting items: 55, 52, 67, 70, 69, 94, 90
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 3

Monkey 5:
  Starting items: 69, 85, 89, 91
  Operation: new = old + 7
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 7

Monkey 6:
  Starting items: 75, 53, 73, 52, 75
  Operation: new = old * 7
  Test: divisible by 2
    If true: throw to monkey 0
    If false: throw to monkey 4

Monkey 7:
  Starting items: 94, 60, 79
  Operation: new = old + 2
  Test: divisible by 3
    If true: throw to monkey 1
    If false: throw to monkey 6"#;
