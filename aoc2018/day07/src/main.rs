extern crate petgraph;
use petgraph::{graphmap::DiGraphMap, Incoming};

const NUM_WORKER: usize = 5;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut graph = DiGraphMap::<_, ()>::from_edges(
        input
            .lines()
            .map(|l| l.chars().filter(|c| c.is_uppercase()).collect::<Vec<_>>())
            .map(|v| (v[0], v[1])),
    );
    let mut second = 0;
    let (mut times_left, mut jobs) = ([0; NUM_WORKER], [None; NUM_WORKER]);
    while graph.node_count() > 0 {
        for (idx, worker) in times_left.iter_mut().enumerate().filter(|(_, s)| **s == 0) {
            if let Some(step) = graph
                .nodes()
                .filter(|n| {
                    graph.edges_directed(*n, Incoming).count() == 0
                        && jobs.iter().filter_map(|c| *c).all(|c| c != *n)
                })
                .min()
            {
                *worker = step.to_digit(36).unwrap() as usize + 51;
                jobs[idx] = Some(step);
            }
        }
        times_left
            .iter_mut()
            .filter(|s| **s != 0)
            .for_each(|s| *s -= 1);
        jobs.iter_mut()
            .zip(times_left.iter())
            .filter_map(|(j, s)| if *s == 0 { Some(j) } else { None })
            .for_each(|j| {
                graph.remove_node(j.unwrap_or_default());
                *j = None;
            });
        second += 1;
    }
    println!("{}", second)
}
