use std::iter::repeat;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut cubes: Vec<Vec<Vec<bool>>> = repeat(
        repeat(repeat(false).take(101).collect())
            .take(101)
            .collect(),
    )
    .take(101)
    .collect();
    for line in input.lines() {
        let (com, range) = line.split_once(' ').unwrap();
        let com = com == "on";
        let mut ranges: Vec<usize> = range
            .split(|c: char| !(c.is_numeric() || c == '-'))
            .filter(|s| !s.is_empty())
            .map(|s| match usize::try_from(s.parse::<isize>().unwrap() + 50) {
                Ok(x @ 0..=100) => x + 1,
                Ok(_) => 102,
                Err(_) => 0,
            })
            .collect();
        let mut end = false;
        for i in (0..6).step_by(2) {
            if ranges[i] == ranges[i+1] && (ranges[i] == 102 || ranges[i] == 0) {
                end = true;
                break;
            }
        }
        if end {
            continue;
        }
        ranges.iter_mut().for_each(|x| *x -= 1);
        (ranges[0]..=ranges[1]).for_each(|x| {
            (ranges[2]..=ranges[3])
                .for_each(|y| (ranges[4]..=ranges[5]).for_each(|z| cubes[x][y][z] = com))
        });
    }
    println!(
        "{}",
        cubes
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter()))
            .filter(|&&x| x)
            .count()
    );
}
