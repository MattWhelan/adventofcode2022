use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

#[derive(Debug)]
struct Map {
    heights: Vec<Vec<u32>>,
    start: [usize; 2],
    end: [usize; 2],
}

impl Map {
    fn new(map_chars: &Vec<Vec<char>>) -> Map {
        let heights = map_chars
            .iter()
            .map(|row| {
                row.iter()
                    .map(|ch| match ch {
                        'S' => 0,
                        'E' => 25,
                        _ => u32::from(*ch) - u32::from('a'),
                    })
                    .collect()
            })
            .collect();

        let start = map_chars
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, ch)| match ch {
                    'S' => Some([x, y]),
                    _ => None,
                })
            })
            .next()
            .unwrap();

        let end = map_chars
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, ch)| match ch {
                    'E' => Some([x, y]),
                    _ => None,
                })
            })
            .next()
            .unwrap();

        Map {
            heights,
            start,
            end,
        }
    }

    fn neighbors(&self, pt: &[usize; 2]) -> impl Iterator<Item = [usize; 2]> {
        let mut left = None;
        let mut right = None;
        let mut up = None;
        let mut down = None;

        let [x, y] = pt;

        if *x > 0 && self[pt] + 1 >= self[&[x - 1, *y]] {
            left = Some([x - 1, *y]);
        }
        if *x + 1 < self.heights[*y].len() && self[pt] + 1 >= self[&[x + 1, *y]] {
            right = Some([x + 1, *y]);
        }
        if *y > 0 && self[pt] + 1 >= self[&[*x, y - 1]] {
            up = Some([*x, y - 1]);
        }
        if *y + 1 < self.heights.len() && self[pt] + 1 >= self[&[*x, y + 1]] {
            down = Some([*x, y + 1]);
        }

        [left, right, up, down].into_iter().filter_map(|x| x)
    }

    fn neighbors_2(&self, pt: &[usize; 2]) -> impl Iterator<Item = [usize; 2]> {
        let mut left = None;
        let mut right = None;
        let mut up = None;
        let mut down = None;

        let [x, y] = pt;

        if *x > 0 && self[pt] <= self[&[x - 1, *y]] + 1 {
            left = Some([x - 1, *y]);
        }
        if *x + 1 < self.heights[*y].len() && self[pt] <= self[&[x + 1, *y]] + 1 {
            right = Some([x + 1, *y]);
        }
        if *y > 0 && self[pt] <= self[&[*x, y - 1]] + 1 {
            up = Some([*x, y - 1]);
        }
        if *y + 1 < self.heights.len() && self[pt] <= self[&[*x, y + 1]] + 1 {
            down = Some([*x, y + 1]);
        }

        [left, right, up, down].into_iter().filter_map(|x| x)
    }

    fn dijkstra(&self) -> u32 {
        let mut visited = HashSet::new();
        let mut distance: HashMap<[usize; 2], u32> = HashMap::new();
        let mut current = self.start;

        distance.insert(current.clone(), 0);

        while !visited.contains(&self.end) {
            let neighbor_dist = distance[&current] + 1;
            self.neighbors(&current)
                .filter(|p| !visited.contains(p))
                .for_each(|p| {
                    let d = distance.entry(p).or_insert(neighbor_dist);
                    if *d > neighbor_dist {
                        *d = neighbor_dist;
                    }
                });
            visited.insert(current.clone());

            current = distance
                .iter()
                .filter(|(p, _)| !visited.contains(&**p))
                .min_by_key(|(_, d)| *d)
                .unwrap()
                .0
                .clone()
        }

        distance[&self.end]
    }

    fn dijkstra_2(&self) -> u32 {
        let mut visited = HashSet::new();
        let mut distance: HashMap<[usize; 2], u32> = HashMap::new();
        let mut current = self.end;

        distance.insert(current.clone(), 0);

        loop {
            let neighbor_dist = distance[&current] + 1;
            self.neighbors_2(&current)
                .filter(|p| !visited.contains(p))
                .for_each(|p| {
                    let d = distance.entry(p).or_insert(neighbor_dist);
                    if *d > neighbor_dist {
                        *d = neighbor_dist;
                    }
                });
            visited.insert(current.clone());

            if self[&current] == 0 {
                break distance[&current];
            }

            current = distance
                .iter()
                .filter(|(p, _)| !visited.contains(&**p))
                .min_by_key(|(_, d)| *d)
                .unwrap()
                .0
                .clone()
        }
    }
}

impl Index<&[usize; 2]> for Map {
    type Output = u32;

    fn index(&self, [x, y]: &[usize; 2]) -> &Self::Output {
        &self.heights[*y][*x]
    }
}

fn main() -> Result<()> {
    let input: Vec<Vec<char>> = INPUT.lines().map(|l| l.chars().collect()).collect();

    let map = Map::new(&input);

    let path_len = map.dijkstra();
    println!("Part 1: {}", path_len);
    let path_len_2 = map.dijkstra_2();
    println!("Part 2: {}", path_len_2);

    Ok(())
}

const INPUT: &str = r#"abcccccccaaaaaaaaccccccccccaaaaaaccccccaccaaaaaaaccccccaacccccccccaaaaaaaaaaccccccccccccccccccccccccccccccccaaaaa
abcccccccaaaaaaaaacccccccccaaaaaacccccaaacaaaaaaaaaaaccaacccccccccccaaaaaaccccccccccccccccccccccccccccccccccaaaaa
abcccccccaaaaaaaaaaccccccccaaaaaacaaacaaaaaaaaaaaaaaaaaaccccccccccccaaaaaaccccccccccccccaaacccccccccccccccccaaaaa
abaaacccccccaaaaaaacccccccccaaacccaaaaaaaaaaaaaaaaaaaaaaaaacccccccccaaaaaaccccccccccccccaaacccccccccccccccccaaaaa
abaaaaccccccaaaccccccccccccccccccccaaaaaaaaacaaaacacaaaaaacccccccccaaaaaaaacccccccccccccaaaaccaaacccccccccccaccaa
abaaaaccccccaaccccaaccccccccccccccccaaaaaaacaaaaccccaaaaaccccccccccccccccacccccccccccccccaaaaaaaaacccccccccccccca
abaaaaccccccccccccaaaacccccccccaacaaaaaaaacccaaacccaaacaacccccccccccccccccccccccccccciiiiaaaaaaaacccccccccccccccc
abaaacccccccccccaaaaaacccccccccaaaaaaaaaaacccaaacccccccaacccccccccccaacccccccccccccciiiiiiijaaaaccccccccaaccccccc
abaaaccccccccccccaaaacccccccccaaaaaaaacaaacccaaaccccccccccccccccccccaaacaaacccccccciiiiiiiijjjacccccccccaaacccccc
abcccccaacaacccccaaaaaccccccccaaaaaacccccacaacccccccccccccccccccccccaaaaaaaccccccciiiinnnoijjjjjjjjkkkaaaaaaacccc
abcccccaaaaacccccaacaaccccccccccaaaacccaaaaaaccccccccccccccccccccccccaaaaaaccccccciiinnnnooojjjjjjjkkkkaaaaaacccc
abccccaaaaacccccccccccccccccccccaccccccaaaaaaaccccccccccccccccccccaaaaaaaaccccccchhinnnnnoooojjooopkkkkkaaaaccccc
abccccaaaaaaccccccccccccccccccccccccccccaaaaaaacccccccccccccccccccaaaaaaaaacccccchhhnnntttuooooooopppkkkaaaaccccc
abccccccaaaaccccccccccacccccccccccccccccaaaaaaacccaaccccccccccccccaaaaaaaaaaccccchhhnnttttuuoooooppppkkkaaaaccccc
abccccccaccccccccccccaaaacaaaccccccccccaaaaaacaaccaacccaaccccccccccccaaacaaacccchhhnnnttttuuuuuuuuupppkkccaaccccc
abccccccccccccccaaccccaaaaaaaccccccccccaaaaaacaaaaaacccaaaaaaccccccccaaacccccccchhhnnntttxxxuuuuuuupppkkccccccccc
abcccccccccccccaaaacccaaaaaaacccaccccccccccaaccaaaaaaacaaaaaaccccccccaacccaaccchhhhnnnttxxxxuuyyyuupppkkccccccccc
abcccccccccccccaaaaccaaaaaaaaacaaacccccccccccccaaaaaaaaaaaaaccccccccccccccaaachhhhmnnnttxxxxxxyyyuvppkkkccccccccc
abcccccccccccccaaaacaaaaaaaaaaaaaaccccccccccccaaaaaacaaaaaaaccccccccccccccaaaghhhmmmttttxxxxxyyyyvvpplllccccccccc
abccacccccccccccccccaaaaaaaaaaaaaaccccccccccccaaaaaacccaaaaaacccaacaacccaaaaagggmmmttttxxxxxyyyyvvppplllccccccccc
SbaaaccccccccccccccccccaaacaaaaaaaacccccccccccccccaacccaaccaacccaaaaacccaaaagggmmmsttxxxEzzzzyyvvvppplllccccccccc
abaaaccccccccccccccccccaaaaaaaaaaaaacaaccccccccccccccccaaccccccccaaaaaccccaagggmmmsssxxxxxyyyyyyvvvqqqlllcccccccc
abaaacccccccccccccccccccaaaaaaaaaaaaaaaaacccccccccccccccccccccccaaaaaaccccaagggmmmsssxxxwywyyyyyyvvvqqlllcccccccc
abaaaaacccccccccccccccccccaacaaaccaaaaaaacccccccccccccccccccccccaaaaccccccaagggmmmssswwwwwyyyyyyyvvvqqqllcccccccc
abaaaaaccccccccccccccccccccccaaaccccaaaacccccccccccccccccaaccaacccaaccccccccgggmmmmssssswwyywwvvvvvvqqqlllccccccc
abaaaaacccccccccccccaccacccccaaaccccaaaacccccccccccccccccaaaaaacccccccccccaaggggmllllsssswwywwwvvvvqqqqlllccccccc
abaaccccccccccccccccaaaaccccccccccccaccaccccccccccccccccccaaaaacccccccccccaaagggglllllssswwwwwrrqqqqqqmmllccccccc
abaaccccccccccccccccaaaaaccccccaaccaaccccccccccccccccccccaaaaaaccaacccccccaaaaggfffllllsswwwwrrrrqqqqqmmmcccccccc
abacaaaccccccccccccaaaaaaccccccaaaaaaccccccaacccccccccccaaaaaaaacaaacaaccccaaaaffffflllsrrwwwrrrmmmmmmmmmcccccccc
abaaaaaccccccccccccaaaaaaccccccaaaaaccccccaaaaccccccccccaaaaaaaacaaaaaaccccaaaaccfffflllrrrrrrkkmmmmmmmccccaccccc
abaaaacccccccccccccccaaccccccccaaaaaacccccaaaacccccccccccccaaccaaaaaaaccccccccccccffflllrrrrrkkkmmmmmccccccaccccc
abaaacccccccccccccccccccccccccaaaaaaaaccccaaaacccccccccccccaaccaaaaaaacccccccccccccfffllkrrrkkkkmddddcccccaaacccc
abaaacccccccccccccccccccccccccaaaaaaaacccccccccccccccccccccccccccaaaaaaccccccccccccfffllkkkkkkkdddddddcaaaaaacccc
abaaaacccccccccccccccccccccccccccaaccccccccccccccccccccccccccccccaacaaacccccccccccccfeekkkkkkkddddddcccaaaccccccc
abcaaacccccccccccaaaccccccccaacccaaccccaaaaaccccaaaccccccccccccccaaccccccccccccccccceeeeekkkkdddddccccccaaccccccc
abccccccccccccccaaaaaaccccccaaacaaccacaaaaaaaccaaaaccccccccccaccaaccccccccccccccccccceeeeeeeedddacccccccccccccccc
abccccccccccccccaaaaaacccccccaaaaacaaaaaccaaaaaaaacccccccccccaaaaacccccccccccccccccccceeeeeeedaaacccccccccccccaaa
abccccccaaacccccaaaaacccccccaaaaaacaaaaaaaaaaaaaaaccccccccccccaaaaaccccccccccccccccccccceeeeecaaacccccccccccccaaa
abccccccaaaccccccaaaaacccccaaaaaaaaccaaaaacaaaaaaccccccccccccaaaaaacccccccccccccccccccccaaaccccaccccccccccccccaaa
abccccaacaaaaacccaaaaacccccaaaaaaaacaaaaaaaaaaaaaaaccccaaaaccaaaacccccccccccccccccccccccaccccccccccccccccccaaaaaa
abccccaaaaaaaaccccccccccccccccaaccccaacaaaaaaaaaaaaaaccaaaaccccaaacccccccccccccccccccccccccccccccccccccccccaaaaaa"#;
