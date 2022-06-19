use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input: Vec<u32> = input.lines().map(|l| l.parse().unwrap()).collect();
    for num1 in input.iter() {
        for num2 in input.iter() {
            if let Some(num3) = input.iter().find(|num3| num1 + num2 + *num3 == 2020) {
                println!("{}", num1 * num2 * num3);
                break;
            }
        }
    }
}
