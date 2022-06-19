use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let algorithm: Vec<_> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect();
    let mut image: HashMap<(isize, isize), bool> = input
        .lines()
        .skip(2)
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .map(move |(y, c)| ((x as isize, y as isize), c == '#'))
        })
        .collect();
    let mut def = false;
    let (mut min, mut xmax, mut ymax) = (
        0,
        *image.keys().map(|(x, _)| x).max().unwrap(),
        *image.keys().map(|(_, y)| y).max().unwrap(),
    );
    let iter = |x, y| {
        [
            (x - 1, y - 1),
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
        .into_iter()
    };
    for a in 0..50 {
        let mut newimage: HashMap<(isize, isize), bool> = HashMap::new();
        min -= 1;
        xmax += 1;
        ymax += 1;
        for x in min..=xmax {
            for y in min..=ymax {
                let alg =
                    algorithm[num(iter(x, y).map(|(x, y)| *image.entry((x, y)).or_insert(def)))];
                newimage.insert((x, y), alg);
            }
        }
        def = algorithm[num(std::iter::repeat(def).take(9))];
        image = newimage;
        println!("{}", a);
    }
    println!("{}", image.values().filter(|&&x| x).count());
}

fn num<I: Iterator<Item = bool>>(iter: I) -> usize {
    iter.fold(0, |acc, x| acc * 2 + x as usize)
}
