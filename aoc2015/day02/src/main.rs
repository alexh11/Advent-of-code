fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!(
        "{}",
        input
            .lines()
            .map(|line| line
                .split('x')
                .map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>())
            .map(|v| [v[0] * v[1], v[0] * v[2], v[1] * v[2]])
            .map(|i| i.iter().sum::<u32>() * 2 + i.iter().min().unwrap())
            .sum::<u32>()
    );
}
