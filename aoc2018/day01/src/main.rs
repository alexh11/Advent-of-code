use std::collections::HashSet;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut reached: HashSet<i32> = HashSet::new();
    println!(
        "{}",
        input
            .lines()
            .map(|x| x.parse::<i32>().unwrap())
            .cycle()
            .scan(0, |acc, x| {
                if reached.insert(*acc) {
                    *acc += x;
                    Some(*acc)
                } else {
                    None
                }
            })
            .last()
            .unwrap()
    );
}
