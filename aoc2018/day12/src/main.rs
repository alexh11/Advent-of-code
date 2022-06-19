fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut input = input.lines();
    let mut plants = std::iter::repeat(false)
        .take(42)
        .chain(
            input
                .next()
                .unwrap()
                .split_once(' ')
                .unwrap()
                .1
                .chars()
                .map(|c| c == '#'),
        )
        .chain(std::iter::repeat(false).take(42))
        .collect::<Vec<_>>();
    input.next();
    let mut rules: std::collections::HashMap<Vec<bool>, bool> = input
        .map(|l| {
            let (pat, res) = l.split_once(" => ").unwrap();
            (pat.chars().map(|c| c == '#').collect(), res == "#")
        })
        .collect();
    for i in 0..20 {
        print!("{}: ", i);
        for plant in plants.iter() {
            print!("{}", if *plant {
                '#'
            } else {
                '.'
            });
        }
        println!("");
        let mut nextgen = plants.clone();
        for (idx, plant) in nextgen.iter_mut().enumerate().skip(2) {
            let mut v: Vec<_> = plants.iter().skip(idx - 2).take(5).map(|b| *b).collect();
            while v.len() < 5 {
                v.push(false);
            }
            *plant = *rules.entry(v).or_insert(false);
        }
        plants = nextgen;
    }
    println!(
        "{}",
        plants
            .iter()
            .enumerate()
            .filter_map(|(i, p)| if *p { Some(i as i32 - 49) } else { None })
            .inspect(|i| println!("{}", i))
            .sum::<i32>()
    );
}
