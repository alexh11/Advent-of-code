fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "{}",
        input
            .lines()
            .map(|l| {
                let l = l.split_whitespace().filter_map(|s| s.parse::<u32>().ok());
                l.clone()
                    .find_map(|x| {
                        if let Some(y) = l.clone().find(|y| x % y == 0 && x != *y) {
                            Some(x / y)
                        } else {
                            None
                        }
                    })
                    .unwrap()
            })
            .inspect(|x| println!("{}", x))
            .sum::<u32>()
    );
}
