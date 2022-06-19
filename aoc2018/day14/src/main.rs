fn main() {
    let input = "286051"
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let mut scores = vec![3, 7];
    let mut elf1 = 0;
    let mut elf2 = 1;
    let mut count = 0;
    loop {
        let sum = scores[elf1] + scores[elf2];
        if sum >= 10 {
            scores.push(sum / 10);
        }
        scores.push(sum % 10);
        elf1 += scores[elf1] + 1;
        elf1 %= scores.len();
        elf2 += scores[elf2] + 1;
        elf2 %= scores.len();
    }
}
