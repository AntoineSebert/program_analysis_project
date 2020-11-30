use crate::parser::Declaration::{Array, Record, Var};
use crate::lexer::{Token, delimiter::Delimiter, keyword::Keyword::*, literal::{IntegerLiteral, Literal}, symbol::Symbol};
use crate::microc::{decl::Declaration, expr::{Expression, BooleanExpr}, stmt::{Scope, Statement}};
use std::collections::linked_list::LinkedList;

pub type Ast = Scope;

fn contains(scope: &Vec<Declaration>, name: &String) -> Option<Declaration> {
	for decl in scope.iter() {
		match decl {
			Var(t, id) if name == id => return Some(Var(*t, id.to_string())),
			Array(t, s, id) if name == id => return Some(Array(*t, s.to_vec(), id.to_string())),
			Record(s, id) if name == id => return Some(Record(s.to_vec(), id.to_string())),
			_ => (),
		}
	}

	None
}

fn parse_lvalueexpr(tokens: &Vec<Token>, i: usize, nested_scope: &LinkedList<Vec<Declaration>>) -> Result<(Expression, usize), String> {
	if i < tokens.len() {
		match tokens[i] {

		}
		Ok()
	} else {
		Err(format!("At least one token is necessary to parse a lvalue expr, 0 found."))
	}
}

fn parse_boolexpr(tokens: &Vec<Token>, i: usize, nested_scope: &LinkedList<Vec<Declaration>>) -> Result<(BooleanExpr, usize), String> {
	Err("".to_string())
}

fn parse_arex(tokens: &Vec<Token>, i: usize, nested_scope: &LinkedList<Vec<Declaration>>) -> Result<(Expression, usize), String> {
	Err("".to_string())
}

fn parse_assign(tokens: &Vec<Token>, i: usize, nested_scope: &LinkedList<Vec<Declaration>>) -> Result<(Statement, usize), String> {
	/*
	LvalueAssign(LvalueExpr, ArithmeticExpr),
	RecordAssign(String, Vec<ArithmeticExpr>),
	*/
	Err("".to_string())
}

fn parse_write(tokens: &Vec<Token>, i: usize, nested_scope: &LinkedList<Vec<Declaration>>) -> Result<(Statement, usize), String> {
	if i + 2 < tokens.len() {
		if let (Token::Keyword(Write), Ok((Expression::ArithmeticExpr(arex), i))) = (&tokens[i], parse_arex(tokens, i + 1, nested_scope)) {
			Ok((Statement::Write(arex), i))
		} else {
			Err(format!("Expected 'Write', ArithmeticExpr, got '{:?}', '{:?}'.", &tokens[i], tokens[i + 1]))
		}
	} else {
		Err(format!("At least three tokens are necessary to parse a write statement, {:?} found.", tokens.len().saturating_sub(i + 2)))
	}
}

fn parse_read(tokens: &Vec<Token>, i: usize, nested_scope: &LinkedList<Vec<Declaration>>) -> Result<(Statement, usize), String> {
	if i + 2 < tokens.len() {
		if let (Token::Keyword(Read), Ok((Expression::LvalueExpr(lvalueexpr), i))) = (&tokens[i], parse_lvalueexpr(tokens, i + 1, nested_scope)) {
			Ok((Statement::Read(lvalueexpr), i))
		} else {
			Err(format!("Expected 'Read', LvalueExpr, got '{:?}', '{:?}'.", &tokens[i], tokens[i + 1]))
		}
	} else {
		Err(format!("At least three tokens are necessary to parse a read statement, {:?} found.", tokens.len().saturating_sub(i + 2)))
	}
}

fn parse_statement_scope(tokens: &Vec<Token>, i: usize, nested_scope: &LinkedList<Vec<Declaration>>, in_loop: bool) -> Result<(Statement, usize), String> {
	if let Token::Delimiter(Delimiter::OpenCurly) = tokens[i] {
		let mut decls = Vec::<Declaration>::new();

		while let Some(_i) = parse_declaration(&tokens, i, &mut decls) {
			i = _i;
		}

		let mut stmts = Vec::<Statement>::new();
		nested_scope.push_back(decls.clone());

		while let Ok((stmt, _i)) = parse_statement(&tokens, i, &nested_scope, in_loop) {
			i = _i;
			stmts.push(stmt);
		}

		if let Token::Delimiter(Delimiter::CloseCurly) = tokens[i] {
			Ok((Statement::Scope((decls, stmts)), i + 1))
		} else {
			Err(format!("Expected 'CloseCurly', got '{:?}'.", &tokens[i]))
		}
	} else {
		Err(format!("Expected 'OpenCurly', got '{:?}'.", &tokens[i]))
	}
}

fn parse_if(tokens: &Vec<Token>, i: usize, nested_scope: &LinkedList<Vec<Declaration>>, in_loop: bool) -> Result<(Statement, usize), String> {
	if let (Token::Keyword(If), Token::Delimiter(Delimiter::OpenPar)) = (tokens[i], tokens[i + 1]) {
		if let Ok((boolex, i)) = parse_boolexpr(&tokens, i + 2, nested_scope) {
			if let (Token::Delimiter(Delimiter::ClosePar), Token::Delimiter(Delimiter::OpenCurly)) = (tokens[i], tokens[i + 1]) {
				if let Ok((Statement::Scope(scope), i)) = parse_statement_scope(tokens, i + 2, nested_scope, in_loop) {
					if let (Token::Keyword(Else), Token::Delimiter(Delimiter::OpenCurly)) = (tokens[i], tokens[i + 1]) {
						if let Ok((Statement::Scope(scope2), i)) = parse_statement_scope(tokens, i + 2, nested_scope, in_loop) {
							Ok((Statement::IfElse(boolex, Box::new(scope), Box::new(scope2)), i + 1))
						} else {
							Err(format!("Cannot parse scope starting with '{:?}'.", &tokens[i + 2]))
						}
					} else {
						Ok((Statement::If(boolex, Box::new(scope)), i + 1))
					}
				} else {
					Err(format!("Cannot parse scope starting with '{:?}'.", &tokens[i + 2]))
				}
			} else {
				Err(format!("Expected 'ClosePar', 'OpenCurly', got '{:?}', '{:?}'.", tokens[i], tokens[i + 1]))
			}
		} else {
			Err(format!("Cannot parse boolean expression starting with '{:?}'.", &tokens[i]))
		}
	} else {
		Err(format!("Expected 'If', 'OpenPar', got '{:?}'", (&tokens[i], &tokens[i + 1])))
	}
}

fn parse_while(tokens: &Vec<Token>, i: usize, nested_scope: &LinkedList<Vec<Declaration>>) -> Result<(Statement, usize), String> {
	if let (Token::Keyword(While), Token::Delimiter(Delimiter::OpenPar)) = (tokens[i], tokens[i + 1]) {
		if let Ok((boolex, i)) = parse_boolexpr(&tokens, i + 2, nested_scope) {
			if let (Token::Delimiter(Delimiter::ClosePar), Token::Delimiter(Delimiter::OpenCurly)) = (tokens[i], tokens[i + 1]) {
				if let Ok((Statement::Scope(scope), i)) = parse_statement_scope(tokens, i + 2, nested_scope, true) {
					Ok((Statement::While(boolex, Box::new(scope)), i + 1))
				} else {
					Err(format!("Cannot parse scope starting with '{:?}'.", &tokens[i + 2]))
				}
			} else {
				Err(format!("Expected 'ClosePar', 'OpenCurly', got '{:?}', '{:?}'.", tokens[i], tokens[i + 1]))
			}
		} else {
			Err(format!("Cannot parse boolean expression starting with '{:?}'.", &tokens[i]))
		}
	} else {
		Err(format!("Expected 'While', 'OpenPar', got '{:?}'", (&tokens[i], &tokens[i + 1])))
	}
}

fn parse_statement(tokens: &Vec<Token>, i: usize, nested_scope: &LinkedList<Vec<Declaration>>, in_loop: bool) -> Result<(Statement, usize), String> {
	match &tokens[i] {
		Token::Keyword(While) => parse_while(tokens, i, nested_scope),
		Token::Keyword(Write) => parse_write(tokens, i, nested_scope),
		Token::Keyword(Read) => parse_read(tokens, i, nested_scope),
		Token::Keyword(If) => parse_if(tokens, i, nested_scope, in_loop),
		Token::Identifier(id) => parse_assign(tokens, i, nested_scope),
		Token::Delimiter(Delimiter::OpenCurly) => parse_statement_scope(tokens, i, nested_scope, in_loop),
		Token::Keyword(Break) => if in_loop {
			Ok((Statement::Break, i + 1))
		} else {
			Err("'Break' keyword only allowed in the body of loops.".to_string())
		},
		Token::Keyword(Continue) => if in_loop {
			Ok((Statement::Continue, i + 1))
		} else {
			Err("'Continue' keyword only allowed in the body of loops.".to_string())
		},
		_ => Err(format!("Cannot parse statement starting with '{:?}'.", &tokens[i]))
	}
}

fn parse_declaration_variable(tokens: &Vec<Token>, i: usize, scope: &mut Vec<Declaration>) -> Result<usize, String> {
	//let min_token_number = 2;

	if i + 2 < tokens.len() {
		if let (
			Token::Keyword(Type(t)), Token::Identifier(id), Token::Symbol(Symbol::Semi)
		) = (&tokens[i], &tokens[i + 1], &tokens[i + 2]) {
			if contains(scope, id).is_some() {
				Err(format!("A variable with the name {:?} is already present in the scope.", id))
			} else {
				scope.push(Var(*t, id.to_string()));
				Ok(i + 3)
			}
		} else {
			Err(format!("Expected 'Type', 'Identifier', 'Semi', got '{:?}'", (&tokens[i], &tokens[i + 1], &tokens[i + 2])))
		}
	} else {
		Err(format!("At least three tokens are necessary to parse a variable, {:?} found.", tokens.len().saturating_sub(i + 2)))
	}
}

fn parse_dimension (tokens: &Vec<Token>, i: usize) -> Option<(IntegerLiteral, usize)> {
	if let Token::Literal(Literal::IntegerLiteral(il)) = &tokens[i] {
		if let Token::Symbol(Symbol::Comma) = &tokens[i + 1] {
			return Some((*il, i + 2))
		} else if let Token::Delimiter(Delimiter::CloseSquare) = &tokens[i + 1] {
			return Some((*il, i + 1))
		}
	}

	None
}

fn parse_declaration_array(tokens: &Vec<Token>, mut i: usize, scope: &mut Vec<Declaration>) -> Result<usize, String> {
	if i + 6 < tokens.len() {
		if let (Token::Keyword(Type(t)), Token::Delimiter(Delimiter::OpenSquare)) = (&tokens[i], &tokens[i + 1]) {
			i += 2;
			let mut dimensions = Vec::<IntegerLiteral>::new();

			while let Some((dim, _i)) = parse_dimension(tokens, i) {
				i = _i;
				dimensions.push(dim);
			}

			if !dimensions.is_empty() {
				if let (
					Token::Delimiter(Delimiter::CloseSquare), Token::Identifier(id), Token::Symbol(Symbol::Semi)
				) = (&tokens[i], &tokens[i + 1], &tokens[i + 2]) {
					if contains(scope, id).is_some() {
						Err(format!("An array with the name {:?} is already present in the scope.", id))
					} else {
						scope.push(Array(*t, dimensions, id.to_string()));
						Ok(i + 3)
					}
				} else {
					Err(format!("Expected 'CloseSquare', 'Identifier', 'Semi', got '{:?}'", (&tokens[i], &tokens[i + 1], &tokens[i + 2])))
				}
			} else {
				Err(format!("Dimensionless array, got '{:?}'.", tokens[i]))
			}
		} else {
			Err(format!("Expected 'Type', 'OpenSquare', got '{:?}'", (&tokens[i], &tokens[i + 1])))
		}
	} else {
		Err(format!("At least six tokens are necessary to parse an array, {:?} found.", tokens.len().saturating_sub(i + 6)))
	}
}

fn parse_declaration_record(tokens: &Vec<Token>, mut i: usize, scope: &mut Vec<Declaration>) -> Result<usize, String> {
	if i + 7 < tokens.len() {
		if let Token::Delimiter(Delimiter::OpenCurly) = &tokens[i] {
			i += 1;
			let mut decls = Vec::<Declaration>::new();

			while let Some(_i) = parse_declaration(tokens, i, &mut decls) {
				i = _i;
			}

			if !decls.is_empty() {
				if let (
					Token::Delimiter(Delimiter::CloseCurly), Token::Identifier(id), Token::Symbol(Symbol::Semi)
				) = (&tokens[i], &tokens[i + 1], &tokens[i + 2]) {
					if contains(scope, id).is_some() {
						Err(format!("A record with the name {:?} is already present in the scope.", id))
					} else {
						scope.push(Record(decls, id.to_string()));
						Ok(i + 3)
					}
				} else {
					Err(format!("Expected 'CloseCurly', 'Identifier', 'Semi', got '{:?}'", (&tokens[i], &tokens[i + 1], &tokens[i + 2])))
				}
			} else {
				Err(format!("Record must contain valid declarations, got '{:?}'", tokens[i]))
			}
		} else {
			Err(format!("Expected 'OpenCurly', found '{:?}'.", tokens[i]))
		}
	} else {
		Err(format!("At least seven tokens are necessary to parse an array, {:?} found.", tokens.len().saturating_sub(i + 7)))
	}
}

fn parse_declaration(tokens: &Vec<Token>, i: usize, scope: &mut Vec<Declaration>) -> Option<usize> {
	if i < tokens.len() {
		if let Token::Keyword(Type(_)) = &tokens[i] {
			if let Ok(i) = parse_declaration_variable(tokens, i, scope) {
				return Some(i);
			} else if let Ok(i) = parse_declaration_array(tokens, i, scope) {
				return Some(i);
			}
		} else if let Token::Delimiter(Delimiter::OpenCurly) = &tokens[i] {
			if let Ok(i) = parse_declaration_record(tokens, i, scope) {
				return Some(i);
			}
		}
	}

	None
}

pub fn parse(tokens: Vec<Token>) -> Result<Ast, String> {
	if tokens.is_empty() {
		Err("No tokens to parse".to_string())
	} else {
		let mut i = 0;
		let mut top_level_scope = Vec::<Declaration>::new();

		while let Some(_i) = parse_declaration(&tokens, i, &mut top_level_scope) {
			i = _i;
		}

		let mut scope_stack = LinkedList::<Vec<Declaration>>::new();
		let mut stmts = Vec::<Statement>::new();
		scope_stack.push_back(top_level_scope.clone());

		while let Ok((stmt, _i)) = parse_statement(&tokens, i, &scope_stack, false) {
			i = _i;
			stmts.push(stmt);
		}

		for decl in top_level_scope.iter() {
			println!("{:?}", decl);
		}

		for stmt in stmts.iter() {
			println!("{:?}", stmt);
		}

		Ok((top_level_scope, stmts))
	}
}