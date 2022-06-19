fn main() {
    let serial = 6303;
    let fuel: Vec<Vec<_>> = (1..=300)
        .map(|x| {
            (1..=300)
                .map(|y| (((x + 10) * y + serial) * (x + 10) / 100) % 10 - 5)
                .collect()
        })
        .collect();
    let (X, Y, size) = (1..=20)
        .inspect(|size| println!("{}", size))
        .flat_map(|size| {
            std::iter::repeat(size)
                .zip(1..=(301 - size))
                .flat_map(|(size, x)| std::iter::repeat((size, x)).zip(1..=(301 - size)))
        })
        .map(|((size, x), y)| (x, y, size))
        .max_by_key(|&(x, y, size)| {
            fuel.iter()
                .skip(x - 1)
                .take(size)
                .flat_map(|v| v.iter().skip(y - 1).take(size))
                .sum::<i32>()
        })
        .unwrap();
    println!("{}, {}, {}", X, Y, size);
}
