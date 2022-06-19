use std::collections::{hash_map::Entry, HashMap, HashSet};
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut set: HashSet<usize> = HashSet::new();
    input
        .lines()
        .map(|line| {
            line.split(|c| !char::is_numeric(c))
                .filter(|s| !s.is_empty())
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| [v[0], v[1], v[1] + v[3], v[2], v[2] + v[4]])
        .for_each(|v| {
            set.insert(v[0]);
            for x in v[1]..v[2] {
                for y in v[2]..v[3] {
                    match map.entry((x, y)) {
                        Entry::Vacant(z) => {
                            z.insert(v[0]);
                        }
                        Entry::Occupied(z) => {
                            set.remove(&v[0]);
                            set.remove(z.get());
                        }
                    }
                }
            }
        });
    assert_eq!(set.iter().count(), 1);
    println!("{}", set.iter().next().unwrap());
}
