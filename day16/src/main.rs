use std::collections::{HashMap, HashSet};
use std::iter::once;
use std::str::FromStr;

use anyhow::{Error, Result};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Valve {
    name: String,
    flow: u32,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let name = caps[1].to_string();
            let flow = caps[2].parse().unwrap();
            let tunnels = caps[3].split(", ").map(|t| t.to_string()).collect();
            Ok(Valve {
                name,
                flow,
                tunnels,
            })
        } else {
            Err(Error::msg("No match"))
        }
    }
}

struct Planner {
    valves: HashMap<String, Valve>,
    node_distances: HashMap<String, HashMap<String, u32>>,
}

impl Planner {
    fn new(valves: &[Valve]) -> Self {
        let valves: HashMap<String, Valve> = valves.iter().cloned().map(|v: Valve| (v.name.to_string(), v)).collect();

        let targets: Vec<_> = valves.values()
            .filter(|v| v.flow > 0)
            .map(|v| v.name.as_str())
            .collect();

        let node_distances: HashMap<String, HashMap<String, u32>> = targets.iter()
            .map(|v| *v)
            .chain(once("AA"))
            .map(|t| (t.to_string(), Planner::dijkstra(&valves, t)))
            .collect();

        Self {
            valves,
            node_distances,
        }
    }

    fn plan(&self) -> u32 {
        let mut known_best: HashMap<(u32, &str, Vec<&str>), (Vec<&str>, u32)> = HashMap::new();
        let targets: Vec<_> = self.valves.values()
            .filter(|v| v.flow > 0)
            .map(|v| v.name.as_str())
            .sorted()
            .collect();

        self.plan_next(30, "AA", targets.clone(), &mut known_best);
        let (p, score) = &known_best[&(30, "AA", targets)];
        dbg!(p);
        *score
    }

    fn plan_next<'a, 'b>(
        &'b self,
        time_left: u32,
        from: &'b str,
        remaining: Vec<&'b str>,
        known_best: &'a mut HashMap<(u32, &'b str, Vec<&'b str>), (Vec<&'b str>, u32)>,
    ) {
        let k = (time_left, from, remaining);
        if !known_best.contains_key(&k) {
            let next_best = k.2.iter()
                .filter_map(|v| {
                    let cost = 1 + &self.node_distances[from][*v];
                    if time_left >= cost {
                        let rest: Vec<_> = k.2.iter().filter(|w| **w != *v).copied().collect();
                        let time = time_left - cost;
                        self.plan_next(
                            time,
                            v,
                            rest.clone(),
                            known_best,
                        );
                        let next_best = &known_best.get(&(time, v, rest));
                        if let Some(next) = next_best {
                            let path: Vec<_> = once(*v).chain(next.0.iter().copied()).collect();
                            let score = self.score(&path, time_left, from);

                            Some((path, score))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .max_by_key(|(_, score)| *score)
                .unwrap_or((Vec::new(), 0));
            known_best.entry(k).or_insert(next_best);
        }
    }

    fn plan_2(&self) -> u32 {
        let mut known_best: HashMap<([u32; 2], [&str; 2], Vec<&str>), (Vec<(&str, u32)>, u32)> = HashMap::new();
        let targets: Vec<_> = self.valves.values()
            .filter(|v| v.flow > 0)
            .map(|v| v.name.as_str())
            .sorted()
            .collect();

        self.plan_next_2([26, 26], ["AA", "AA"], targets.clone(), &mut known_best);
        let (p, score) = &known_best[&([26, 26], ["AA", "AA"], targets)];
        dbg!(p);
        *score
    }

    fn plan_next_2<'a, 'b>(
        &'b self,
        time_left: [u32; 2],
        from: [&'b str; 2],
        remaining: Vec<&'b str>,
        known_best: &'a mut HashMap<([u32; 2], [&'b str; 2], Vec<&'b str>), (Vec<(&'b str, u32)>, u32)>,
    ) {
        let k = (time_left, from, remaining);
        if !known_best.contains_key(&k) {
            let next_best = k.2.iter()
                .filter_map(|v| {
                    let i = if time_left[0] < time_left[1] {
                        1
                    } else {
                        0
                    };
                    let cost = 1 + &self.node_distances[from[i]][*v];
                    let time_budget = time_left[i];
                    let pos = {
                        let mut pos = from.clone();
                        pos[i] = v;
                        pos
                    };
                    if time_budget >= cost {
                        let rest: Vec<_> = k.2.iter().filter(|w| **w != *v).copied().collect();
                        let time = {
                            let mut time = time_left.clone();
                            time[i] = time_budget - cost;
                            time
                        };
                        self.plan_next_2(
                            time,
                            pos,
                            rest.clone(),
                            known_best,
                        );
                        let next_best = &known_best.get(&(time, pos, rest));
                        if let Some(next) = next_best {
                            let step = (*v, time_budget - cost);
                            let path: Vec<_> = once(step).chain(next.0.iter().copied()).collect();
                            let score = self.score_2(&path);

                            Some((path, score))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .max_by_key(|(_, score)| *score)
                .unwrap_or((Vec::new(), 0));
            known_best.entry(k).or_insert(next_best);
        }
    }

    fn score(&self, plan: &[&str], time_left: u32, from: &str) -> u32 {
        let mut at = from;

        let mut score = 0;
        let mut t = time_left;
        for &target in plan {
            let distances = &self.node_distances[at];
            if t > distances[target] + 1 {
                t -= distances[target] + 1;
                score += self.valves[target].flow * t;
                at = target;
            } else {
                break;
            }
        }

        score
    }

    fn score_2(&self, plan: &[(&str, u32)]) -> u32 {
        plan.iter()
            .map(|&(v, t)| self.valves[v].flow * t)
            .sum()
    }

    fn neighbors<'a>(valves: &'a HashMap<String, Valve>, v: &str) -> impl Iterator<Item=&'a str> {
        valves[v].tunnels.iter().map(|s| s.as_str())
    }

    fn dijkstra<'a>(valves: &'a HashMap<String, Valve>, start: &'a str) -> HashMap<String, u32> {
        let mut unvisited: HashSet<&str> = valves.keys().map(|s| s.as_str()).collect();
        let mut distance: HashMap<&str, u32> = valves.keys().map(|k| (k.as_str(), u32::MAX)).collect();
        let mut current = start;

        *distance.entry(start).or_default() = 0;

        loop {
            let neighbor_dist = distance[&current] + 1;
            Planner::neighbors(valves, current)
                .filter(|p| unvisited.contains(p))
                .for_each(|p| {
                    let d = distance.entry(p).or_insert(neighbor_dist);
                    if *d > neighbor_dist {
                        *d = neighbor_dist;
                    }
                });
            unvisited.remove(current);

            if unvisited.is_empty() {
                break;
            }

            if let Some((next, d)) = distance
                .iter()
                .filter(|(p, _)| unvisited.contains(&**p))
                .min_by_key(|(_, d)| *d) {
                if *d == u32::MAX {
                    break;
                }
                current = next
            } else {
                unreachable!()
            }
        }

        distance.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }
}

fn main() -> Result<()> {
    // let input: Vec<Valve> = TEST.lines().map(|l| l.parse().unwrap()).collect();
    let input: Vec<Valve> = INPUT.lines().map(|l| l.parse().unwrap()).collect();

    let planner = Planner::new(&input);

    let score = planner.plan();
    println!("Part 1: {}", score);
    // low 1447

    let score2 = planner.plan_2();
    println!("Part 2: {}", score2);

    Ok(())
}

const TEST: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

// VR 11
// KZ 18
// AJ 6
// VG 25
// IR 10
// SO 22
// SC 14
// RO 19
// JL 21
// PW 9
// DI 23
// JD 15
// SP 24
// OM 7
// RI 3

const INPUT: &str = r#"Valve VR has flow rate=11; tunnels lead to valves LH, KV, BP
Valve UV has flow rate=0; tunnels lead to valves GH, RO
Valve OH has flow rate=0; tunnels lead to valves AJ, NY
Valve GD has flow rate=0; tunnels lead to valves TX, PW
Valve NS has flow rate=0; tunnels lead to valves AJ, AA
Valve KZ has flow rate=18; tunnels lead to valves KO, VK, PJ
Valve AH has flow rate=0; tunnels lead to valves ZP, DI
Valve SA has flow rate=0; tunnels lead to valves VG, JF
Valve VK has flow rate=0; tunnels lead to valves RO, KZ
Valve GB has flow rate=0; tunnels lead to valves XH, AA
Valve AJ has flow rate=6; tunnels lead to valves IC, OH, ZR, NS, EM
Valve PJ has flow rate=0; tunnels lead to valves KZ, SP
Valve KO has flow rate=0; tunnels lead to valves KZ, LE
Valve AA has flow rate=0; tunnels lead to valves TW, GB, TI, NS, UL
Valve TW has flow rate=0; tunnels lead to valves TU, AA
Valve VG has flow rate=25; tunnel leads to valve SA
Valve BP has flow rate=0; tunnels lead to valves RO, VR
Valve XH has flow rate=0; tunnels lead to valves GB, RI
Valve TX has flow rate=0; tunnels lead to valves RI, GD
Valve IR has flow rate=10; tunnels lead to valves TN, NY, JF
Valve TU has flow rate=0; tunnels lead to valves JD, TW
Valve KC has flow rate=0; tunnels lead to valves SP, RO
Valve LN has flow rate=0; tunnels lead to valves EM, RI
Valve HD has flow rate=0; tunnels lead to valves FE, SC
Valve KE has flow rate=0; tunnels lead to valves OM, RI
Valve VY has flow rate=0; tunnels lead to valves PW, BS
Valve LH has flow rate=0; tunnels lead to valves OM, VR
Valve EM has flow rate=0; tunnels lead to valves AJ, LN
Valve SO has flow rate=22; tunnels lead to valves ZP, FE
Valve EC has flow rate=0; tunnels lead to valves OM, UL
Valve KV has flow rate=0; tunnels lead to valves SP, VR
Valve FE has flow rate=0; tunnels lead to valves SO, HD
Valve TI has flow rate=0; tunnels lead to valves AA, PW
Valve SC has flow rate=14; tunnel leads to valve HD
Valve ZP has flow rate=0; tunnels lead to valves SO, AH
Valve RO has flow rate=19; tunnels lead to valves UV, BP, VK, KC
Valve ZR has flow rate=0; tunnels lead to valves OM, AJ
Valve JL has flow rate=21; tunnels lead to valves GN, TN
Valve PW has flow rate=9; tunnels lead to valves TI, GN, VY, GD, IC
Valve UL has flow rate=0; tunnels lead to valves EC, AA
Valve GN has flow rate=0; tunnels lead to valves JL, PW
Valve TN has flow rate=0; tunnels lead to valves JL, IR
Valve NV has flow rate=0; tunnels lead to valves RI, JD
Valve DI has flow rate=23; tunnels lead to valves LE, AH
Valve IC has flow rate=0; tunnels lead to valves PW, AJ
Valve JF has flow rate=0; tunnels lead to valves SA, IR
Valve LE has flow rate=0; tunnels lead to valves DI, KO
Valve BS has flow rate=0; tunnels lead to valves JD, VY
Valve JD has flow rate=15; tunnels lead to valves NV, TU, BS
Valve SP has flow rate=24; tunnels lead to valves KC, KV, PJ
Valve NY has flow rate=0; tunnels lead to valves IR, OH
Valve OM has flow rate=7; tunnels lead to valves EC, GH, KE, ZR, LH
Valve GH has flow rate=0; tunnels lead to valves OM, UV
Valve RI has flow rate=3; tunnels lead to valves NV, KE, LN, XH, TX"#;
