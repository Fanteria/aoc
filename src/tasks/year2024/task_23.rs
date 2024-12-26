use crate::tasks::TaskRun;
use ahash::{AHashMap as HashMap, AHashSet as HashSet};

pub struct Task23;

fn read_graph(input: &str) -> (HashSet<&str>, HashMap<&str, HashSet<&str>>) {
    input
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .fold(
            (HashSet::new(), HashMap::new()),
            |(mut nodes, mut edges), (from, to)| {
                nodes.insert(from);
                nodes.insert(to);
                edges.entry(from).or_default().insert(to);
                edges.entry(to).or_default().insert(from);
                (nodes, edges)
            },
        )
}

fn bron_kerbosch2<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    req: &mut HashSet<&'a str>,
    mut keys: HashSet<&'a str>,
    mut todo: HashSet<&'a str>,
    max_clique: &mut HashSet<&'a str>,
) {
    if keys.is_empty() {
        if todo.is_empty() && req.len() > max_clique.len() {
            *max_clique = req.clone();
        }
        return;
    }
    while let Some(neighbour) = keys.iter().copied().next() {
        let neighbours = &graph[neighbour];
        let new_keys = keys.intersection(neighbours).copied().collect();
        let new_todo = todo.intersection(neighbours).copied().collect();

        req.insert(neighbour);
        bron_kerbosch2(graph, req, new_keys, new_todo, max_clique);
        req.remove(neighbour);

        keys.remove(neighbour);
        todo.insert(neighbour);
    }
}

impl TaskRun for Task23 {
    fn normal(input: &str) -> usize
    where
        Self: Sized,
    {
        let (nodes, edges) = read_graph(input);
        nodes
            .into_iter()
            .filter(|n| n.starts_with("t"))
            .flat_map(|n| {
                edges[n]
                    .iter()
                    .flat_map(|n2| {
                        edges[n2].iter().filter_map(|n3| {
                            edges[n3].iter().find(|n4| **n4 == n).map(|_| {
                                let mut vec = [n, *n2, *n3];
                                vec.sort();
                                vec
                            })
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>()
            .len()
    }

    fn bonus(input: &str) -> usize
    where
        Self: Sized,
    {
        let (nodes, edges) = read_graph(input);
        let mut max_cliques = HashSet::new();

        bron_kerbosch2(
            &edges,
            &mut HashSet::new(),
            nodes,
            HashSet::new(),
            &mut max_cliques,
        );

        let mut clique: Vec<_> = max_cliques.into_iter().collect();
        clique.sort();
        println!("{}", clique.join(","));
        0
    }
}
