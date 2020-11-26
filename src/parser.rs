use crate::parser::Declaration::Record;
use crate::lexer::literal::IntegerLiteral;
use crate::parser::Declaration::{Array, Var};
use crate::lexer::{Token, delimiter::Delimiter, keyword::Keyword::Type, literal::Literal, symbol::Symbol};
use crate::microc::{decl::{Declaration}, stmt::{Scope, Statement}};

pub type Ast = Scope;

fn parse_statement(tokens: &Vec<Token>, i: &usize) -> Option<(Statement, usize)> {
	/*
	match tokens[*i] {
		Token::Identifier(id) => None&tokens[*i],
		_ => None,
	}
	*/
	None
}

fn parse_declaration_variable(tokens: &Vec<Token>, i: usize) -> Option<(Declaration, usize)> {
	if let Token::Keyword(kw) = &tokens[i] {
		if let Type(t) = kw {
			if let Token::Identifier(id) = &tokens[i + 1] {
				if let Token::Symbol(s) = &tokens[i + 2] {
					if *s == Symbol::Semi {
						return Some((Var(*t, id.to_string()), i + 3));
					}
				}
			}
		}
	}

	None
}

fn parse_dimension (tokens: &Vec<Token>, i: usize) -> Option<(IntegerLiteral, usize)> {
	if let Token::Literal(l) = &tokens[i] {
		println!("{:?}", *l);
		if let Literal::IntegerLiteral(il) = l {
			println!("{:?}", *il);
			if let Token::Symbol(s) = &tokens[i + 1] {
				if *s == Symbol::Colon {
					return Some((*il, i + 2))
				}
			} else if let Token::Delimiter(d) = &tokens[i + 1] {
				if *d == Delimiter::CloseSquare {
					return Some((*il, i + 1))
				}
			}
		}
	}

	None
}

fn parse_declaration_array(tokens: &Vec<Token>, i: usize) -> Option<(Declaration, usize)> {
	let mut ii = i;

	if let Token::Keyword(kw) = &tokens[i] {
		if let Type(t) = kw {
			ii += 1;
			println!("array attempt: {:?}", t);

			if let Token::Delimiter(d) = &tokens[ii] {
				if *d == Delimiter::OpenSquare {
					ii += 1;
					let mut dimensions = Vec::<IntegerLiteral>::new();
					println!("{:?}", *d);

					while let Some((dim, _i)) = parse_dimension(tokens, ii) {
						ii = _i;
						dimensions.push(dim);
					}

					if dimensions.len() != 0 {
						if let Token::Delimiter(d) = &tokens[ii] {
							if *d == Delimiter::CloseSquare {
								if let Token::Identifier(id) = &tokens[ii + 1] {
									if let Token::Symbol(s) = &tokens[ii + 2] {
										if *s == Symbol::Semi {
											return Some((Array(*t, dimensions, id.to_string()), ii + 2))
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}

	None
}

fn parse_declaration_record(tokens: &Vec<Token>, i: usize) -> Option<(Declaration, usize)> {
	let mut ii = i;

	if let Token::Delimiter(d) = &tokens[i] {
		if *d == Delimiter::OpenCurly {
			ii += 1;
			let mut decls = Vec::<Declaration>::new();

			while let Some((decl, _i)) = parse_declaration(tokens, ii + 1) {
				ii = _i;
				decls.push(decl);
			}

			if decls.len() != 0 {
				if let Token::Delimiter(d) = &tokens[ii] {
					if *d == Delimiter::CloseCurly {
						if let Token::Identifier(id) = &tokens[ii + 1] {
							if let Token::Symbol(s) = &tokens[ii + 2] {
								if *s == Symbol::Semi {
									return Some((Record(decls, id.to_string()), ii + 2))
								}
							}
						}
					}
				}
			}
		}
	}

	None
}

fn parse_declaration(tokens: &Vec<Token>, i: usize) -> Option<(Declaration, usize)> {
	if let Token::Keyword(kw) = &tokens[i] {
		if let Type(_) = kw {
			if let Some(var_i) = parse_declaration_variable(tokens, i) {
				return Some(var_i);
			} else if let Some(array_i) = parse_declaration_array(tokens, i) {
				return Some(array_i);
			}
		}
	} else if let Some(record_i) = parse_declaration_record(tokens, i) {
		return Some(record_i);
	}

	None
}

pub fn parse(tokens: Vec<Token>) -> Ast {
	let mut program: Ast = (Vec::<Declaration>::new(), Vec::<Statement>::new());

	if tokens.len() != 0 {
		let mut i = 0;

		while let Some((decl, _i)) = parse_declaration(&tokens, i) {
			i = _i;
			program.0.push(decl);
		}

		while let Some((stmt, _i)) = parse_statement(&tokens, &i) {
			i = _i;
			program.1.push(stmt);
		}
	}

	for decl in program.0.iter() {
		println!("{:?}", decl);
	}

	for stmt in program.1.iter() {
		println!("{:?}", stmt);
	}

	program
}