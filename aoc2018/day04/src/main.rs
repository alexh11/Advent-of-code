use std::collections::HashMap;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut current_guard = 0;
    let mut fell_asleep = 0;
    let mut times_asleep: HashMap<usize, Vec<Vec<usize>>> = HashMap::new();
    for line in input.lines() {
        if let Some(i) = line.chars().position(|c| c == '#') {
            current_guard = line
                .chars()
                .skip(i + 1)
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();
        } else if line.contains('f') {
            fell_asleep = line[15..17].parse().unwrap();
        } else {
            times_asleep
                .entry(current_guard)
                .or_insert(vec![])
                .push((fell_asleep..line[15..17].parse().unwrap()).collect());
        }
    }
    let (guard, (minute, _)) = times_asleep
        .iter()
        .map(|(g, v)| {
            (
                g,
                (0..60)
                    .map(|x| (x, v.iter().filter(|v2| v2.iter().any(|y| *y == x)).count()))
                    .max_by_key(|&(_, c)| c)
                    .unwrap(),
            )
        })
        .max_by_key(|&(_, (_, c))| c)
        .unwrap();
    println!("{}", guard * minute);
}
