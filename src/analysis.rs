use crate::worklist::Worklist;
use crate::worklist::FifoWorklist;
use crate::flow_graph::{FlowGraph, Action};
use std::{collections::HashMap, cmp::PartialEq};
use petgraph::{graph::NodeIndex, Direction};

trait Analyzer<R> {
	fn map(&self, a: Action) -> R;
}

#[derive(Debug, PartialEq)]
pub enum Sign {
	Plus,
	Minus,
	Zero,
}

struct SignDetecter {}

impl Analyzer<Sign> for SignDetecter {
	fn map(&self, a: Action) -> Sign {
		match a {
			_ => Sign::Plus,
		}
	}
}

fn builder<W: Worklist<NodeIndex> + Default>(worklist: &W, program: FlowGraph) -> W {
	let mut new_worklist = W::default();

	new_worklist.insert(program.1);

	for child in program.0.neighbors_directed(program.1, Direction::Outgoing) {
		if !worklist.contains(child) {
			builder::<W>(worklist, (program.0.clone(), child));
		}
	}

	new_worklist
}

fn worklist<W: Worklist<NodeIndex>, R: std::cmp::PartialEq, A: Analyzer<R>>(program: FlowGraph, specification: A) -> HashMap<NodeIndex, R> {
	let mut wl = W::default();
	let mut res = HashMap::<NodeIndex, R>::new();
	let (graph, start) = program;

	let mut children = builder::<W>(&wl, (graph.clone(), start));

	while let Some(node) = children.extract() {
		if wl.insert(node).is_some() {
			res.insert(start, specification.map(graph[start].clone()));
		}
	}

	while let Some(node) = wl.extract() {
		let old = res.get(&node).unwrap();
		let new = specification.map(graph[node].clone());

		if *old != new {
			for child in graph.neighbors_directed(node, Direction::Outgoing) {
				wl.insert(child);
			}
		}
	}

	res
}

pub fn analyze(program: FlowGraph, analysis: String) -> Result<HashMap<NodeIndex, Sign>, String> {
	if program.0.node_count() == 0 {
		Err("The flow graph is empty.".to_string())
	} else {
		let sd = SignDetecter {};

		let result = worklist::<FifoWorklist<NodeIndex>, Sign, SignDetecter>(program, sd);

		Ok(result)
	}
}
