fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut layout: Vec<Vec<Option<bool>>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'L' => Some(false),
                    '#' => Some(true),
                    _ => None,
                })
                .collect()
        })
        .collect();
    let offsets = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    println!(
        "{}",
        loop {
            let mut newlayout = layout.clone();
            let mut changed = false;
        }
    );
}

fn iter_offsets_2D<'a, I, Offset, Ind, Outer, Inner, Data>(
    iter: I,
    (x, y): (Ind, Ind),
    slice: &'a Outer,
) -> Vec<&'a Data>
where
    I: Iterator<Item = (Offset, Offset)>,
    Outer: std::slice::SliceIndex<Ind, Output = &'a Inner>,
    Inner: std::slice::SliceIndex<Ind, Output = &'a Data> + 'a,
    Offset: std::convert::TryInto<Ind> + Copy + std::ops::Add<Output = Offset>,
    Ind: std::convert::TryInto<Offset> + Copy,
{
    iter.filter_map(|(dx, dy)| {
        Some(
            slice
                .get(&(x.try_into().ok()? + dx).try_into().ok()?)?
                .get(&(y.try_into().ok()? + dy).try_into().ok()?)?,
        )
    })
    .collect()
}
