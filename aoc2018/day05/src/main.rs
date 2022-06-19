fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input.trim();
    let polymer: Vec<i8> = input
        .chars()
        .map(|c| {
            c.to_digit(36).unwrap_or_else(|| panic!("{}", c)) as i8
                * match c.is_uppercase() {
                    true => 1,
                    false => -1,
                }
        })
        .collect();
    let mut min = usize::MAX;
    for rem in 10..36 {
        println!("{}", rem);
        let mut polymer = polymer.clone();
        polymer.retain(|x| x.abs() != rem);
        while let Some(i) = polymer.windows(2).position(|w| w[0] == -w[1]) {
            polymer.drain(i..i + 2);
        }
        if polymer.len() < min {
            min = polymer.len();
        }
    }
    println!("{}", min);
}
