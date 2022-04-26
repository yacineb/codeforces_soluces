
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};
use std::io;

// I chose to represent the graph using adjacency list
// in the current problem the graph is a positively-weighted undirected graph (distances are psitive)
type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;

// adds an edge with weight
fn add_edge(graph: &mut Graph<i32, i32>, v1: i32, v2: i32, weight: i32) {
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, weight);
    graph.entry(v2).or_insert_with(BTreeMap::new);
}

// performs Dijsktra's algorithm on the given graph from the given start
/// https://fr.wikipedia.org/wiki/Algorithme_de_Dijkstra
/// This returns a map that for each reachable vertex associates the distance from start and the predecessor
pub fn shortest_path(
    graph: &Graph<i32, i32>,
    start: &i32,
) -> BTreeMap<i32, Option<(i32, i32)>> /* BTreeMap<INTERSECTION_ID, predecessor = Option(INTERSECTION_ID, distance)> */ {
    let mut dists = BTreeMap::new();

    // priority queue
    let mut prio = BinaryHeap::new();

    // start is the only one which does not have a predessessor
    dists.insert(*start, None);

    for (new, weight) in &graph[start] {
        dists.insert(*new, Some((*start, *weight)));
        prio.push(Reverse((*weight, new, start)));
    }

    while let Some(Reverse((dist_new, new, prev))) = prio.pop() {
        match dists[new] {
            // what we popped is what is in dists, we'll compute it
            Some((p, d)) if p == *prev && d == dist_new => {}
            _ => continue, // otherwise skip
        }

        for (next, weight) in &graph[new] {
            match dists.get(next) {
                // if dist[next] is a lower dist than the alternative one, we do nothing
                Some(Some((_, dist_next))) if dist_new + *weight >= *dist_next => {}
                // if dist[next] is None then next is start and so the distance won't be changed, it won't be added again in prio
                Some(None) => {}
                // case when the new path is shorter
                _ => {
                    dists.insert(*next, Some((*new, *weight + dist_new)));
                    prio.push(Reverse((*weight + dist_new, next, new)));
                }
            }
        }
    }

    // println!("{:?}", dists);
    dists
}

// computes optimal distances from start
fn compute_optimal_energies(n: i32, shortcuts: Vec<i32>) -> Vec<i32> {
    // build graph
    let mut graph: Graph<i32, i32> = BTreeMap::new();
    for i in 1..=n {
        for j in 1..=n {
            // how the weight (the vertice length) has to be compute
            let weight = if shortcuts[i as usize - 1]  == j {
                std::cmp::min(1, (i - j).abs())
            }
            else {
                (i - j).abs()
            };
            add_edge(&mut graph, i, j, weight);
        }
    }

    // collect shortest paths from the start
    let res: Vec<_>= shortest_path(&graph, &1).iter().map(|(_, x)| { x.unwrap_or_default().1}).collect();

    println!("{}", res.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(" "));
    res
}


// well , i would have done a better error management using THE trio Ok, Err , Result
fn parse_args() -> (i32, Vec<i32>) {
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("read error");

    let lines: Vec<_> = a_str.lines().collect();
    let n = lines[0].parse::<i32>().unwrap();

    let shortcuts: Vec<_> = lines[1].split_whitespace().map(|item| item.parse::<i32>().unwrap()).collect();
    (n, shortcuts)
}

fn main() {
    let (n, shortcuts) = parse_args();
    compute_optimal_energies(n, shortcuts);
     // assert_eq!(compute_optimal_energies(7, vec![4,4,4,4,7,7,7]), vec![0,1,2,1,2,3,3]);
    //assert_eq!(compute_optimal_energies(5, vec![1,2,3,4,5]), vec![0,1,2,3,4 ])

}
