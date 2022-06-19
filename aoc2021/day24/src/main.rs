fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let input = input
        .split("inp w")
        .filter(|x| !x.is_empty())
        .map(|x| "inp w".to_string() + x)
        .collect::<Vec<_>>();
    let input: Vec<Vec<_>> = input.iter().map(|x| x.lines().collect()).collect();
    let mut s = "".to_string();
    for i in 0..input[0].len() {
        for j in 0..input.len() {
            s += input[j][i];
            s += ",";
        }
        s += "\n";
    }
    std::fs::write("/mnt/c/Users/heatl/Documents/output.csv", s).unwrap();
}

z /= (1 or 26);
if z % 26 + num == inp {
    z += inp + num2;
}
else {
    z *= 26;
}
