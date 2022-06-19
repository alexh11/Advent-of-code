fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut iter = input
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok());
    println!("{}", node_value(&mut iter));
}

fn node_value<I>(iter: &mut I) -> usize
where
    I: Iterator<Item = usize>,
{
    let nodes = iter.next().unwrap();
    let metas = iter.next().unwrap();
    let mut out = 0;
    if nodes == 0 {
        for _ in 0..metas {
            out += iter.next().unwrap();
        }
        out
    } else {
        let mut node_vals = vec![];
        for _ in 0..nodes {
            node_vals.push(node_value(iter));
        }
        for _ in 0..metas {
            out += node_vals.get(iter.next().unwrap() - 1).unwrap_or(&0);
        }
        out
    }
}
