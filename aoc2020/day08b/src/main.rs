fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut prog: Vec<_> = input.lines().map(|x| x.to_owned()).collect();
    let mut change = prog.len();
    println!(
        "{}",
        loop {
            let result;
            change -= 1;
            if prog[change].contains("jmp") {
                prog[change] = prog[change].replace("jmp", "nop");
                result = run(&prog);
                prog[change] = prog[change].replace("nop", "jmp");
            } else if prog[change].contains("nop") {
                prog[change] = prog[change].replace("nop", "jmp");
                result = run(&prog);
                prog[change] = prog[change].replace("jmp", "nop");
            } else {
                continue;
            }
            if let Some(x) = result {
                break x;
            }
        }
    );
}

fn run(prog: &Vec<String>) -> Option<i32> {
    let mut acc = 0;
    let mut cur = 0;
    let mut visited = std::collections::HashSet::new();
    loop {
        if !visited.insert(cur) {
            break None;
        }
        if cur == prog.len() as i32 {
            break Some(acc);
        }
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
}
