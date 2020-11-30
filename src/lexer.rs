use std::{convert::TryFrom, fs::File, io::{BufReader, BufRead}, path::Path, vec::Vec};

pub mod delimiter {
	use std::{convert::TryFrom, fmt::{self, Display, Formatter}};

	#[derive(Debug, PartialEq)]
	pub enum Delimiter {
		OpenCurly,
		OpenSquare,
		OpenPar,
		CloseCurly,
		CloseSquare,
		ClosePar,
	}

	impl Display for Delimiter {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use Delimiter::*;

			match *self {
				OpenCurly => write!(f, "{{"),
				OpenSquare => write!(f, "["),
				OpenPar => write!(f, "("),
				CloseCurly => write!(f, "}}"),
				CloseSquare => write!(f, "]"),
				ClosePar => write!(f, ")"),
			}
		}
	}

	impl TryFrom<char> for Delimiter {
		type Error = String;

		fn try_from(value: char) -> Result<Self, Self::Error> {
			use Delimiter::*;

			match value {
				'{' => Ok(OpenCurly),
				'[' => Ok(OpenSquare),
				'(' => Ok(OpenPar),
				'}' => Ok(CloseCurly),
				']' => Ok(CloseSquare),
				')' => Ok(ClosePar),
				_ => Err(format!("Unknown delimiter '{value}'."))
			}
		}
	}
}

pub mod keyword {
	use std::{convert::TryFrom, fmt::{self, Display, Formatter}};

	#[derive(Debug, PartialEq, Clone, Copy)]
	pub enum Type {
		Int,
		Float,
		Bool,
	}

	impl Display for Type {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use Type::*;

			match *self {
				Int => write!(f, "int"),
				Float => write!(f, "float"),
				Bool => write!(f, "bool"),
			}
		}
	}

	impl TryFrom<String> for Type {
		type Error = String;

		fn try_from(value: String) -> Result<Self, Self::Error> {
			use Type::*;

			match value.as_str() {
				"int" => Ok(Int),
				"float" => Ok(Float),
				"bool" => Ok(Bool),
				_ => Err(format!("Unknown type '{value}'."))
			}
		}
	}

	#[derive(Debug, PartialEq)]
	pub enum Keyword {
		Break,
		Continue,
		Else,
		False,
		If,
		Read,
		True,
		While,
		Write,
		Type(Type),
	}

	impl Display for Keyword {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use Keyword::*;

			match &*self {
				Break => write!(f, "break"),
				Continue => write!(f, "continue"),
				Else => write!(f, "else"),
				False => write!(f, "false"),
				If => write!(f, "if"),
				Read => write!(f, "read"),
				True => write!(f, "true"),
				While => write!(f, "while"),
				Write => write!(f, "write"),
				Type(t) => write!(f, "{t}"),
			}
		}
	}

	impl TryFrom<String> for Keyword {
		type Error = String;

		fn try_from(value: String) -> Result<Self, Self::Error> {
			use Keyword::*;
			use crate::lexer::keyword;

			match value.as_str() {
				"break" => Ok(Break),
				"continue" => Ok(Continue),
				"else" => Ok(Else),
				"false" => Ok(False),
				"if" => Ok(If),
				"read" => Ok(Read),
				"true" => Ok(True),
				"while" => Ok(While),
				"write" => Ok(Write),
				_ => if let Ok(t) = keyword::Type::try_from(value.clone()) {
					Ok(Type(t))
				} else {
					Err(format!("Unknown keyword '{value}'."))
				}
			}
		}
	}
}

pub mod literal {
	use std::{convert::TryFrom, fmt::{self, Display, Formatter}};

	#[derive(Debug, Clone, Copy)]
	pub enum IntegerLiteral {
		DecimalLiteral(isize),
		BinaryLiteral(isize),
		OctalLiteral(isize),
		HexadecimalLiteral(isize),
	}

	impl Display for IntegerLiteral {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use IntegerLiteral::*;

			match *self {
				DecimalLiteral(n) => write!(f, "{n}"),
				BinaryLiteral(n) => write!(f, "{:#b}", n),
				OctalLiteral(n) => write!(f, "{:#o}", n),
				HexadecimalLiteral(n) => write!(f, "{:#x}", n),
			}
		}
	}

	impl TryFrom<String> for IntegerLiteral {
		type Error = String;

		fn try_from(value: String) -> Result<Self, Self::Error> {
			use IntegerLiteral::*;

			match value.parse::<isize>() {
				Ok(n)  => Ok(DecimalLiteral(n)),
				Err(_) => if value.starts_with("0b") {
					Ok(BinaryLiteral(isize::from_str_radix(&value[2..], 2).unwrap()))
				} else if value.starts_with("0o") {
					Ok(OctalLiteral(isize::from_str_radix(&value[2..], 8).unwrap()))
				} else if value.starts_with("0x") {
					Ok(HexadecimalLiteral(isize::from_str_radix(&value[2..], 16).unwrap()))
				} else {
					Err(format!("Unknown integer literal '{value}'."))
				},
			}
		}
	}

	#[derive(Debug)]
	pub enum Literal {
		IntegerLiteral(IntegerLiteral),
		FloatLiteral(f64),
		BooleanLiteral(bool),
	}

	impl Display for Literal {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use Literal::*;

			match &*self {
				IntegerLiteral(i) => write!(f, "{i}"),
				FloatLiteral(_f) => write!(f, "{_f}"),
				BooleanLiteral(b) => write!(f, "{b}"),
			}
		}
	}

	impl TryFrom<String> for Literal {
		type Error = String;

		fn try_from(value: String) -> Result<Self, Self::Error> {

			if let Ok(l) = IntegerLiteral::try_from(value.clone()) {
				Ok(Literal::IntegerLiteral(l))
			} else if let Ok(l) = value.parse::<f64>() {
				Ok(Literal::FloatLiteral(l))
			} else if let Ok(l) = value.parse::<bool>() {
				Ok(Literal::BooleanLiteral(l))
			} else {
				Err(format!("Unknown literal '{value}'."))
			}
		}
	}
}

pub mod symbol {
	use std::{convert::TryFrom, fmt::{self, Display, Formatter}};

	#[derive(Debug, PartialEq, Clone)]
	pub enum Symbol {
		Plus,			// Addition
		Minus,			// Subtraction, Negation
		Star,			// Multiplication
		Slash,			// Division
		Percent,		// Remainder
		Caret,			// Bitwise and Logical XOR
		Not,			// Bitwise and Logical NOT
		And,			// Bitwise and Logical AND
		Or,				// Bitwise and Logical OR
		AndAnd,			// Lazy AND
		OrOr,			// Lazy OR
		Shl,			// Shift Left
		Shr,			// Shift Right
		PlusEq,			// Addition assignment
		MinusEq,		// Subtraction assignment
		StarEq,			// Multiplication assignment
		SlashEq,		// Division assignment
		PercentEq,		// Remainder assignment
		CaretEq,		// Bitwise XOR assignment
		AndEq,			// Bitwise And assignment
		OrEq,			// Bitwise Or assignment
		ShlEq,			// Shift Left assignment
		ShrEq,			// Shift Right assignment
		Eq,				// Assignment
		EqEq,			// Equal
		Ne,				// Not Equal
		Gt,				// Greater than
		Lt,				// Less than
		Ge,				// Greater than or equal to
		Le,				// Less than or equal to
		Dot,			// Field access, Tuple index
		Comma,			// Various separators
		Semi,			// Terminator for various items and statements, Array types
		Colon,			// Various separators
		ColonEq,		// Variable assignment
	}

	impl Display for Symbol {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use Symbol::*;

			match *self {
				Plus => write!(f, "+"),
				Minus => write!(f, "-"),
				Star => write!(f, "*"),
				Slash => write!(f, "*"),
				Percent => write!(f, "%"),
				Caret => write!(f, "^"),
				Not => write!(f, "!"),
				And => write!(f, "&"),
				Or => write!(f, "|"),
				AndAnd => write!(f, "&&"),
				OrOr => write!(f, "||"),
				Shl => write!(f, "<"),
				Shr => write!(f, ">"),
				PlusEq => write!(f, "+="),
				MinusEq => write!(f, "-="),
				StarEq => write!(f, "*="),
				SlashEq => write!(f, "/="),
				PercentEq => write!(f, "%="),
				CaretEq => write!(f, "^="),
				AndEq => write!(f, "&="),
				OrEq => write!(f, "|="),
				ShlEq => write!(f, "<="),
				ShrEq => write!(f, ">="),
				Eq => write!(f, "="),
				EqEq => write!(f, "=="),
				Ne => write!(f, "!="),
				Gt => write!(f, ">"),
				Lt => write!(f, "<"),
				Ge => write!(f, ">="),
				Le => write!(f, "<="),
				Dot => write!(f, "."),
				Comma => write!(f, ","),
				Semi => write!(f, ";"),
				Colon => write!(f, ":"),
				ColonEq => write!(f, ":="),
			}
		}
	}

	impl TryFrom<String> for Symbol {
		type Error = String;

		fn try_from(value: String) -> Result<Self, Self::Error> {
			use Symbol::*;

			match value.as_str() {
				"+" => Ok(Plus),
				"-" => Ok(Minus),
				"*" => Ok(Star),
				"/" => Ok(Slash),
				"%" => Ok(Percent),
				"^" => Ok(Caret),
				"!" => Ok(Not),
				"&" => Ok(And),
				"|" => Ok(Or),
				"&&" => Ok(AndAnd),
				"||" => Ok(OrOr),
				"<<" => Ok(Shl),
				">>" => Ok(Shr),
				"+=" => Ok(PlusEq),
				"-=" => Ok(MinusEq),
				"*=" => Ok(StarEq),
				"/=" => Ok(SlashEq),
				"%=" => Ok(PercentEq),
				"^=" => Ok(CaretEq),
				"&=" => Ok(AndEq),
				"|=" => Ok(OrEq),
				"<<=" => Ok(ShlEq),
				">>=" => Ok(ShrEq),
				"=" => Ok(Eq),
				"==" => Ok(EqEq),
				"!=" => Ok(Ne),
				">" => Ok(Gt),
				"<" => Ok(Lt),
				">=" => Ok(Ge),
				"<=" => Ok(Le),
				"." => Ok(Dot),
				"," => Ok(Comma),
				";" => Ok(Semi),
				":" => Ok(Colon),
				":=" => Ok(ColonEq),
				_ => Err(format!("Unknown symbol '{value}'.")),
			}
		}
	}

	impl TryFrom<char> for Symbol {
		type Error = String;

		fn try_from(value: char) -> Result<Self, Self::Error> {
			use Symbol::*;

			match value {
				'+' => Ok(Plus),
				'-' => Ok(Minus),
				'*' => Ok(Star),
				'/' => Ok(Slash),
				'%' => Ok(Percent),
				'^' => Ok(Caret),
				'!' => Ok(Not),
				'&' => Ok(And),
				'|' => Ok(Or),
				'=' => Ok(Eq),
				'>' => Ok(Gt),
				'<' => Ok(Lt),
				'.' => Ok(Dot),
				',' => Ok(Comma),
				';' => Ok(Semi),
				':' => Ok(Colon),
				_ => Err(format!("Unknown single-char symbol '{value}'.")),
			}
		}
	}

	impl TryFrom<Vec<Symbol>> for Symbol {
		type Error = String;

		fn try_from(value: Vec<Symbol>) -> Result<Self, Self::Error> {
			use Symbol::*;

			if value.len() == 2 {
				if value[0] == Gt && value[1] == Gt {
					Ok(Shr)
				} else if value[0] == Lt && value[1] == Lt {
					Ok(Shl)
				} else if value[1] == Eq {
					match value[0] {
						Plus => Ok(PlusEq),
						Minus => Ok(MinusEq),
						Star => Ok(StarEq),
						Slash => Ok(SlashEq),
						Percent => Ok(PercentEq),
						Caret => Ok(CaretEq),
						Not => Ok(Ne),
						And => Ok(AndEq),
						Or => Ok(OrEq),
						Eq => Ok(EqEq),
						Gt => Ok(Ge),
						Lt => Ok(Le),
						Colon => Ok(ColonEq),
						_ => Err(format!("Unknown digraph start symbol '{:?}'.", value)),
					}
				} else {
					Err(format!("Unknown digraph operator '{:?}'.", value))
				}
			} else if 3 == value.len() {
				if value[0] == Gt && value[1] == Gt && value[2] == Eq {
					Ok(ShlEq)
				} else if value[0] == Lt && value[1] == Lt && value[2] == Eq {
					Ok(ShrEq)
				} else {
					Err(format!("Unknown trigraph symbol '{:?}'.", value))
				}
			} else {
				Err(format!("Unknown multigraph symbol '{:?}'.", value))
			}
		}
	}
}

#[derive(Debug)]
pub enum Token {
	Delimiter(delimiter::Delimiter),
	Identifier(String),
	Keyword(keyword::Keyword),
	Literal(literal::Literal),
	Symbol(symbol::Symbol),
}

pub fn lex(path: &Path) -> Result<Vec<Token>, String> {
	let mut tokens = Vec::<Token>::new();
	let reader = BufReader::new(File::open(path).unwrap());
	let mut line_pos = 0;
	let mut multiline_comment = false;

	for line in reader.lines().map(|l| l.unwrap()) {
		let _line: Vec<char> = line.trim().chars().collect();
		line_pos += 1;
		let mut i = 0;

		// multiline comment
		if multiline_comment {
			while i < _line.len() {
				if i < _line.len() - 1 && _line[i] == '*' && _line[i + 1] == '/' {
					multiline_comment = false;
					i += 2;
					break;
				}
				i += 1;
			}
		}

		if !multiline_comment {
			while i < _line.len() {
				// boolean literal, keyword, identifier
				if _line[i].is_alphabetic() {
					for ii in i.._line.len() {
						if !(_line[ii].is_alphanumeric() || _line[ii] == '_') {
							let buff: String = _line[i..ii].iter().collect();

							if let Ok(kw) = keyword::Keyword::try_from(buff.clone()) {
								if kw == keyword::Keyword::True || kw == keyword::Keyword::False {
									tokens.push(Token::Literal(literal::Literal::try_from(kw.to_string())?));
								} else {
									tokens.push(Token::Keyword(kw));
								}
							} else {
								tokens.push(Token::Identifier(buff.clone()));
							}

							i = ii - 1;
							break;
						}
					}
				// int literal & float literal
				} else if _line[i].is_numeric() {
					for ii in i.._line.len() {
						if !_line[ii].is_alphanumeric() {
							tokens.push(Token::Literal(literal::Literal::try_from(_line[i..ii].iter().collect::<String>())?));
							i = ii - 1;
							break;
						}
					}
				// symbols
				} else if let Ok(s) = symbol::Symbol::try_from(_line[i]) {
					// comments
					if s == symbol::Symbol::Slash {
						if _line[i + 1] == '/' {
							break;
						} else if _line[i + 1] == '*' {
							multiline_comment = true;
							i += 1;
						}
					// punctuation
					} else if s == symbol::Symbol::Dot || s == symbol::Symbol::Comma || s == symbol::Symbol::Semi {
						tokens.push(Token::Symbol(s));
					// multigraphs and simple ops
					} else {
						let mut buff = Vec::<symbol::Symbol>::new();
						buff.push(s);

						for ii in (i + 1).._line.len() {
							if let Ok(o) = symbol::Symbol::try_from(_line[ii]) {
								buff.push(o);
							} else {
								i = ii - 1;
								break;
							}
						}

						while !buff.is_empty() {
							match buff.len() {
								1 => tokens.push(Token::Symbol(buff.remove(0))),
								2 => if let Ok(d) = symbol::Symbol::try_from(buff[0..=1].to_vec()) {
									buff.drain(0..=1);
									tokens.push(Token::Symbol(d))
								} else {
									tokens.push(Token::Symbol(buff.remove(0)))
								},
								_ => if let Ok(t) = symbol::Symbol::try_from(buff[0..=2].to_vec()) {
									buff.drain(0..=2);
									tokens.push(Token::Symbol(t))
								} else if let Ok(d) = symbol::Symbol::try_from(buff[0..=1].to_vec()) {
									buff.drain(0..=1);
									tokens.push(Token::Symbol(d))
								} else {
									tokens.push(Token::Symbol(buff.remove(0)))
								},
							}
						}
					}
				// delimiter
				} else if let Ok(d) = delimiter::Delimiter::try_from(_line[i]) {
					tokens.push(Token::Delimiter(d))
				}

				i += 1;
			}
		}
	}

	/*
	for token in &tokens {
		println!("{:?}", token);
	}
	*/

	Ok(tokens)
}