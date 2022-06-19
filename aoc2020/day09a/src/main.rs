fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut last: Vec<_> = input
        .lines()
        .take(25)
        .map(|l| l.parse::<usize>().unwrap())
        .collect();
    for line in input.lines().skip(25) {
        let num = line.parse::<usize>().unwrap();
        if !last
            .iter()
            .flat_map(|x| last.iter().map(move |y| x + y))
            .any(|x| x == num)
        {
            println!("{}", num);
            break;
        }
        last.remove(0);
        last.push(num);
    }
}
