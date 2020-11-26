use crate::{microc::{decl::Declaration, stmt::Statement}, parser::Ast};
use petgraph::graph::{DiGraph, NodeIndex};

#[derive(Debug, Clone)]
pub enum Action {
	Declaration(Declaration),
	Statement(Statement),
}

pub type FlowGraph = (DiGraph<Action, u32>, NodeIndex<u32>);
/*
fn edges<T: Declaration + Statement>(T: token) -> Action {
}
*/

/// Constructs the program graph for a program in MicroC
pub fn flow(program: Ast) -> FlowGraph {
	/*
	match program {
		(decls, stmts) if !decls.empty() && stms
		Some(expr) => expr,
		None => expr,
	}
	*/
	(DiGraph::<Action, u32>::new(), NodeIndex::<u32>::new(0))
}
