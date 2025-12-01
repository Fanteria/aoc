use crate::tasks::TaskRun;
use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use anyhow::{Context, Result};
use itertools::Itertools;
use std::fmt::Display;

pub struct Day23;

#[derive(Default)]
struct Graph<'a> {
    nodes: HashSet<&'a str>,
    edges: HashMap<&'a str, HashSet<&'a str>>,
}

fn read_graph(input: &str) -> Result<Graph<'_>> {
    input
        .lines()
        .map(|line| line.split_once("-").context("Missing split character '-'"))
        .try_fold(Graph::default(), |mut g, splitted| {
            let (from, to) = splitted?;
            g.nodes.insert(from);
            g.nodes.insert(to);
            g.edges.entry(from).or_default().insert(to);
            g.edges.entry(to).or_default().insert(from);
            Ok::<_, anyhow::Error>(g)
        })
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

impl TaskRun for Day23 {
    fn normal(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        let Graph { nodes, edges } = read_graph(input)?;
        Ok(nodes
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
            .len())
    }

    fn bonus(input: &str) -> Result<impl Display>
    where
        Self: Sized,
    {
        let Graph { nodes, edges } = read_graph(input)?;
        let mut max_cliques = HashSet::new();

        bron_kerbosch2(
            &edges,
            &mut HashSet::new(),
            nodes,
            HashSet::new(),
            &mut max_cliques,
        );

        Ok(max_cliques.into_iter().sorted().join(","))
    }
}
