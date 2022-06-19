fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut input = input.lines().collect::<Vec<&str>>();
    let mut done = false;
    let mut bags = vec!["shiny gold"];
    while !done {
        input.retain(|x| !bags.iter().any(|y| x.starts_with(y)));
        let mut new: Vec<_> = input
            .iter()
            .filter(|line| bags.iter().any(|bag| line.contains(bag)))
            .map(|line| &line[0..line.match_indices(' ').nth(1).unwrap().0])
            .collect();
        done = new.len() == 0;
        bags.append(&mut new);
    }
    println!("{}", bags.len() - 1);
}
