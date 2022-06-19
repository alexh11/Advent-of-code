extern crate pathfinding;
use pathfinding::prelude::dijkstra;

struct Cave(Vec<Vec<u32>>);

impl Cave {
    fn index(&self, index: (usize, usize)) -> u32 {
        let (x, y) = index;
        let offset = x / self.0.len() + y / self.0[0].len();
        (self.0[x % self.xlen()][y % self.ylen()] - 1 + offset as u32) % 9 + 1
    }
    fn xlen(&self) -> usize {
        self.0.len()
    }
    fn ylen(&self) -> usize {
        self.0[0].len()
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let cave = Cave(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    println!(
        "{}",
        dijkstra(
            &(0, 0),
            |&(x, y): &(usize, usize)| vec![
                (Some(x), Some(y + 1)),
                (Some(x + 1), Some(y)),
                (x.checked_sub(1), Some(y)),
                (Some(x), y.checked_sub(1)),
            ]
            .into_iter()
            .filter_map(|(x, y)| {
                if x? < cave.xlen() * 5 && y? < cave.ylen() * 5 {
                    Some((x?, y?))
                } else {
                    None
                }
            })
            .map(|p| (p, cave.index(p))),
            |&(x, y)| x == cave.xlen() * 5 - 1 && y == cave.ylen() * 5 - 1
        )
        .unwrap()
        .1
    );
}
