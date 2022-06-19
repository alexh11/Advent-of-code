fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut last: Vec<_> = input.iter().take(25).collect();
    let mut num_found = 0;
    for num in input.iter().skip(25) {
        if !last
            .iter()
            .flat_map(|&&x| last.iter().map(move |&&y| x + y))
            .any(|x| x == *num)
        {
            num_found = *num;
            break;
        }
        last.remove(0);
        last.push(num);
    }
    let thing = (2..input.len())
        .flat_map(|len| {
            let input = &input;
            (0..=(input.len() - len)).map(move |begin| {
                (
                    input.iter().skip(begin).take(len).sum::<usize>(),
                    input.iter().skip(begin).take(len).max().unwrap()
                        + input.iter().skip(begin).take(len).min().unwrap(),
                )
            })
        })
        .find(|(sum, _)| *sum == num_found)
        .unwrap()
        .1;
    println!("{}", thing);
}
