fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut points: Vec<_> = input
        .lines()
        .map(|l| {
            l.split(',')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .map(|v| (v[0], v[1], v[2], v[3]))
        .collect();
    let mut constellations: Vec<Vec<(i32, i32, i32, i32)>> = vec![];
    while points.len() > 0 {
        constellations.push(vec![points.remove(0)]);
        while let Some(i) = points
            .iter()
            .position(|p| constellations.last().unwrap().iter().any(|x| dist(*x, *p) <= 3))
        {
            constellations.last_mut().unwrap().push(points.remove(i));
        }
    }
    println!("{}", constellations.len());
}

fn dist((w0, x0, y0, z0): (i32, i32, i32, i32), (w1, x1, y1, z1): (i32, i32, i32, i32)) -> i32 {
    (w0 - w1).abs() + (x0 - x1).abs() + (y0 - y1).abs() + (z0 - z1).abs()
}
