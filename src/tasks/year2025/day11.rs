use crate::{tasks::TaskRun, utils::{NodeId, OrientedGraph}};
use anyhow::{Context, Result};
use std::{collections::HashMap, fmt::Display};

pub struct Day11;

impl Day11 {
    fn read_input(input: &str) -> Result<OrientedGraph<&str>> {
        input
            .lines()
            .try_fold(OrientedGraph::default(), |mut graph, line| {
                let (node, targets) = line.split_once(": ").context("Failed to read node")?;
                let id = graph.get_or_insert_node(node);
                targets.split_whitespace().for_each(|target| {
                    let target = graph.get_or_insert_node(target);
                    graph.add_edge(&id, target);
                });
                Ok(graph)
            })
    }

    fn dfs(
        graph: &OrientedGraph<&str>,
        paths: &mut HashMap<NodeId, usize>,
        cur: &NodeId,
        end: &NodeId,
    ) -> usize {
        if let Some(p) = paths.get(cur) {
            *p
        } else if *cur == *end {
            1
        } else {
            let total = graph
                .get_edges(cur)
                .iter()
                .map(|next| Self::dfs(graph, paths, next, end))
                .sum();
            paths.insert(*cur, total);
            total
        }
    }
}

impl TaskRun for Day11 {
    fn normal(input: &str) -> Result<impl Display> {
        let graph = Self::read_input(input)?;
        let srv = graph.get_node_id(&"you").unwrap();
        let out = graph.get_node_id(&"out").unwrap();
        Ok(Self::dfs(&graph, &mut HashMap::default(), srv, out))
    }

    fn bonus(input: &str) -> Result<impl Display> {
        let graph = Self::read_input(input)?;
        let srv = graph.get_node_id(&"svr").unwrap();
        let dac = graph.get_node_id(&"dac").unwrap();
        let fft = graph.get_node_id(&"fft").unwrap();
        let out = graph.get_node_id(&"out").unwrap();
        //   srv -> dac -> fft -> out
        Ok((Self::dfs(&graph, &mut HashMap::default(), srv, dac)
        * Self::dfs(&graph, &mut HashMap::default(), dac, fft)
        * Self::dfs(&graph, &mut HashMap::default(), fft, out))
        //   srv -> fft -> dac -> out
        + (Self::dfs(&graph, &mut HashMap::default(), srv, fft)
            * Self::dfs(&graph, &mut HashMap::default(), fft, dac)
            * Self::dfs(&graph, &mut HashMap::default(), dac, out)))
    }
}
