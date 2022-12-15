use std::ops::Range;
use anyhow::{Error, Result};
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Sensor {
    pos: [i64; 2],
    beacon: [i64; 2],
}

impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)").unwrap();
        }

        if let Some(caps) = RE.captures(s) {
            let sx: i64 = caps[1].parse().unwrap();
            let sy: i64 = caps[2].parse().unwrap();
            let bx: i64 = caps[3].parse().unwrap();
            let by: i64 = caps[4].parse().unwrap();
            Ok(Sensor {
                pos: [sx, sy],
                beacon: [bx, by]
            })
        } else {
            Err(Error::msg("No match"))
        }
    }
}

impl Sensor {
    fn manh_dist(a: &[i64; 2], b: &[i64; 2]) -> i64 {
        (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
    }

    fn cant_be_beacon(&self, pt: &[i64; 2]) -> bool {
        if pt == &self.pos {
            true
        } else if pt == &self.beacon {
            false
        } else {
            let p_dist = Sensor::manh_dist(&self.pos, pt);
            let b_dist = Sensor::manh_dist(&self.pos, &self.beacon);
            p_dist <= b_dist
        }
    }

    fn could_be_beacon(&self, pt: &[i64; 2]) -> bool {
        let p_dist = Sensor::manh_dist(&self.pos, pt);
        let b_dist = Sensor::manh_dist(&self.pos, &self.beacon);
        p_dist > b_dist
    }

    fn outline(&self, bounds: Range<i64>) -> impl Iterator<Item=[i64; 2]> + '_{
        let r = Sensor::manh_dist(&self.pos, &self.beacon)+1;

        (-r..=r).flat_map(move |dx| {
            let dy = (r-dx).abs();
            [
                [self.pos[0]+dx, self.pos[1]+dy],
                [self.pos[0]+dx, self.pos[1]-dy],
            ]
        }).filter(move |pt| bounds.contains(&pt[0]) && bounds.contains(&pt[1]))
    }
}

fn main() -> Result<()> {
    let input: Vec<Sensor> = INPUT.lines().map(|l| l.parse().unwrap()).collect();

    {
        let min_x = input.iter().flat_map(|s| [s.pos[0], s.beacon[0]]).min().unwrap();
        let max_x = input.iter().flat_map(|s| [s.pos[0], s.beacon[0]]).max().unwrap();
        let rng = max_x - min_x;

        let non_beacon_points = (min_x - rng..max_x + rng).map(|x| [x, 2000000])
            .filter(|pt| input.iter().any(|s| s.cant_be_beacon(&pt)))
            .count();

        println!("Part 1: {}", non_beacon_points);
    }
    {
        // let limit = 20;
        let limit = 4_000_000;
        if let Some(distress_beacon) = input.iter().flat_map(|s| s.outline(0..limit+1))
            .find(|pt| input.iter().all(|s| s.could_be_beacon(pt))) {
            println!("Part 2: {}", distress_beacon[0]*4000000 + distress_beacon[1]);
        } else {
            println!("Fail");
        }
    }
    Ok(())
}

const INPUT: &str = r#"Sensor at x=3391837, y=2528277: closest beacon is at x=3448416, y=2478759
Sensor at x=399473, y=1167503: closest beacon is at x=1188862, y=2000000
Sensor at x=3769110, y=2896086: closest beacon is at x=4076658, y=2478123
Sensor at x=900438, y=3835648: closest beacon is at x=-435606, y=3506717
Sensor at x=2913762, y=3937542: closest beacon is at x=2964244, y=3612685
Sensor at x=3646459, y=3446878: closest beacon is at x=3264675, y=3635510
Sensor at x=1182092, y=2135147: closest beacon is at x=1188862, y=2000000
Sensor at x=3213897, y=2710772: closest beacon is at x=3448416, y=2478759
Sensor at x=3242113, y=3984214: closest beacon is at x=3264675, y=3635510
Sensor at x=2809237, y=3782833: closest beacon is at x=2872059, y=3592616
Sensor at x=2962421, y=37354: closest beacon is at x=3358601, y=-1111474
Sensor at x=3456740, y=2458922: closest beacon is at x=3448416, y=2478759
Sensor at x=1799203, y=3569221: closest beacon is at x=2872059, y=3592616
Sensor at x=3907873, y=3898376: closest beacon is at x=3264675, y=3635510
Sensor at x=3481951, y=2453964: closest beacon is at x=3448416, y=2478759
Sensor at x=1120077, y=2963237: closest beacon is at x=1188862, y=2000000
Sensor at x=2901181, y=3029961: closest beacon is at x=2872059, y=3592616
Sensor at x=3111105, y=3361570: closest beacon is at x=2964244, y=3612685
Sensor at x=2533601, y=3956413: closest beacon is at x=2872059, y=3592616
Sensor at x=108898, y=2275290: closest beacon is at x=1188862, y=2000000
Sensor at x=3501591, y=2414995: closest beacon is at x=3448416, y=2478759
Sensor at x=3035657, y=3700769: closest beacon is at x=2964244, y=3612685
Sensor at x=1286795, y=298997: closest beacon is at x=308571, y=-434280
Sensor at x=200812, y=3470019: closest beacon is at x=-435606, y=3506717
Sensor at x=2550124, y=1556776: closest beacon is at x=1188862, y=2000000
Sensor at x=3955070, y=601908: closest beacon is at x=4076658, y=2478123
Sensor at x=3565419, y=2355172: closest beacon is at x=3448416, y=2478759"#;
