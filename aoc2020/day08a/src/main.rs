fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let prog: Vec<_> = input.lines().collect();
    let mut acc = 0;
    let mut cur = 0;
    let mut visited = std::collections::HashSet::new();
    while visited.insert(cur) {
        let (op, arg) = prog[cur as usize].split_once(' ').unwrap();
        match op {
            "jmp" => {
                cur += arg.parse::<i32>().unwrap();
            }
            "acc" => {
                acc += arg.parse::<i32>().unwrap();
                cur += 1;
            }
            "nop" => cur += 1,
            _ => unreachable!(),
        }
    }
    println!("{}", acc);
}
