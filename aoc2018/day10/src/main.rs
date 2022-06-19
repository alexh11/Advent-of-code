fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (mut positions, velocities): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<_>>()
        })
        .map(|v| ((v[0], v[1]), (v[2], v[3])))
        .unzip();
    let mut size = square_size(&positions).unwrap();
    let mut time_elapsed = 0;
    loop {
        for ((xpos, ypos), (xvel, yvel)) in positions.iter_mut().zip(velocities.iter()) {
            *xpos += xvel;
            *ypos += yvel;
        }
        let newsize = square_size(&positions).unwrap();
        if newsize > size {
            for ((xpos, ypos), (xvel, yvel)) in positions.iter_mut().zip(velocities.iter()) {
                *xpos -= xvel;
                *ypos -= yvel;
            }
            break;
        }
        size = newsize;
        time_elapsed += 1;
    }
    println!("Seconds elapsed: {}", time_elapsed);
    for y in positions.iter().map(|(_, y)| *y).min().unwrap()
        ..=positions.iter().map(|(_, y)| *y).max().unwrap()
    {
        for x in positions.iter().map(|(x, _)| *x).min().unwrap()
            ..=positions.iter().map(|(x, _)| *x).max().unwrap()
        {
            print!(
                "{}",
                if positions.iter().any(|(xpos, ypos)| *xpos == x && *ypos == y) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!("");
    }
}

fn square_size(pos: &Vec<(i128, i128)>) -> Option<i128> {
    Some(
        (pos.iter().map(|(x, _)| x).max()? - pos.iter().map(|(x, _)| x).min()?)
            * (pos.iter().map(|(_, y)| y).max()? - pos.iter().map(|(_, y)| y).min()?),
    )
}
