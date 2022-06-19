fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input.lines().collect::<Vec<&str>>();
    println!("{}", bag_count(&input, "shiny gold"));
}

fn bag_count(rules: &Vec<&str>, bag: &str) -> u32 {
    let rule = rules.iter().find(|rule| rule.starts_with(bag)).unwrap();
    rule.match_indices(char::is_numeric)
        .map(|(i, num)| (num.parse::<u32>().unwrap(), i + 2))
        .zip(rule.match_indices("bag").skip(1).map(|(i, _)| i - 1))
        .map(|((num, lb), ub)| (num, lb, ub))
        .map(|(num, lb, ub)| num * (bag_count(rules, &rule[lb..ub]) + 1))
        .sum::<u32>()
}
