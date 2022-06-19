fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut input = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    input.sort();
    let mut differences = [0, 0, 1];
    differences[input[0] - 1] += 1;
    input
        .iter()
        .zip(input.iter().skip(1))
        .for_each(|(&a, &b)| differences[b - a - 1] += 1);
    println!("{}", differences[0] * differences[2]);
}
