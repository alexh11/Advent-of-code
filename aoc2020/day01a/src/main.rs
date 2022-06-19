use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    for num1 in input.iter() {
        if let Some(num2) = input.iter().find(|num2| num1 + *num2 == 2020) {
            println!("{}", num1 * num2);
            break;
        }
    }
}
