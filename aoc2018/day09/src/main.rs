fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let (players, last) = (input[0], input[1]);
    let mut players = vec![0; players];
    let mut cur = 0;
    let mut tur
    println!("{}", players.iter().max().unwrap())
}
