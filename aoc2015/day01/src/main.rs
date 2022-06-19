fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "{}",
        input
            .chars()
            .map(|c| match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            })
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .enumerate()
            .find(|(_, x)| *x == -1)
            .unwrap()
            .0
            + 1
    );
}
