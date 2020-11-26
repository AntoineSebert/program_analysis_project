#![feature(arbitrary_enum_discriminant)]
#![feature(format_args_capture)]

use crate::{analysis::analyze, lexer::lex, parser::parse, flow_graph::flow};
use structopt::StructOpt;
use std::path::PathBuf;

mod microc;
mod flow_graph;
mod parser;
mod analysis;
mod lexer;
mod worklist;

/// patterns:
/// - reaching definitions (rd)
/// - sign analysis (sa)
/// - ...
/// - * (all)
///
#[derive(StructOpt)]
struct Cli {
	/// The pattern to look for
	analysis: String,
	/// The path to the file to read
	#[structopt(parse(from_os_str))]
	path: PathBuf,
}

fn main() {
	println!("Hello, world!");
	let args = Cli::from_args();

	if let Ok(tokens) = lex(&args.path.as_path()) {
		let ast = parse(tokens);
		let fg = flow(ast);
		analyze(fg, args.analysis);
	}
}
