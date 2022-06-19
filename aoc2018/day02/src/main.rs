fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "{}",
        input
            .lines()
            .flat_map(|s| std::iter::repeat(s).zip(input.lines()))
            .map(|(s1, s2)| {
                s1.chars()
                    .zip(s2.chars())
                    .fold(("".to_string(), 0), |(s, c), (c1, c2)| {
                        if c1 == c2 {
                            (s + &c1.to_string(), c)
                        } else {
                            (s, c + 1)
                        }
                    })
            })
            .find(|(_, c)| *c == 1)
            .unwrap()
            .0
    );
}
