fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let iter = input.chars().filter_map(|c| c.to_digit(10));
    println!(
        "{}",
        iter.clone().zip(iter.cycle().skip(input.len() / 2))
            .filter_map(|(a, b)| if a == b { Some(a) } else { None })
            .sum::<u32>()
    );
}
