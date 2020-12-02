pub mod ops {
	use std::{convert::TryFrom, fmt::{self, Display, Formatter}};

	#[derive(Debug, Clone)]
	pub enum BinaryOp {
		BitAnd,
		BitOr,
		BitXor,
		Not,
		Shl,
		Shr,
	}

	impl Display for BinaryOp {

		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use BinaryOp::*;

			match *self {
				BitAnd => write!(f, "&"),
				BitOr => write!(f, "|"),
				BitXor => write!(f, "^"),
				Not => write!(f, "!"),
				Shl => write!(f, "<<"),
				Shr => write!(f, ">>"),
			}
		}
	}

	impl TryFrom<String> for BinaryOp {
		type Error = String;

		fn try_from(value: String) -> Result<Self, Self::Error> {
			use BinaryOp::*;

			match value.as_str() {
				"&" => Ok(BitAnd),
				"|" => Ok(BitOr),
				"^" => Ok(BitXor),
				"!" => Ok(Not),
				"<<" => Ok(Shl),
				">>" => Ok(Shr),
				_ => Err(format!("Unknown binary operator '{value}'."))
			}
		}
	}

	#[derive(Debug, Clone)]
	pub enum ArithmeticOp {
		Add,
		Sub,
		Mul,
		Neg,
		Div,
		Rem,
	}

	impl Display for ArithmeticOp {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use ArithmeticOp::*;

			match *self {
				Add => write!(f, "+"),
				Div => write!(f, "/"),
				Mul => write!(f, "*"),
				Neg => write!(f, "-"),
				Rem => write!(f, "%"),
				Sub => write!(f, "-"),
			}
		}
	}

	impl TryFrom<String> for ArithmeticOp {
		type Error = String;

		fn try_from(value: String) -> Result<Self, Self::Error> {
			use ArithmeticOp::*;

			match value.as_str() {
				"+" => Ok(Add),
				"-" => Err(format!("Context needed to parse either Subtraction or Negation operation.")),
				"*" => Ok(Mul),
				"/" => Ok(Div),
				"%" => Ok(Rem),
				_ => Err(format!("Unknown arithmetic operator '{value}'."))
			}
		}
	}

	#[derive(Debug, Clone)]
	pub enum RelationalOp {
		Lt,
		Leq,
		Gt,
		Geq,
		Eq,
		Neq,
	}

	impl Display for RelationalOp {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use RelationalOp::*;

			match *self {
				Lt => write!(f, "<"),
				Leq => write!(f, "<="),
				Gt => write!(f, ">"),
				Geq => write!(f, ">="),
				Eq => write!(f, "=="),
				Neq => write!(f, "!="),
			}
		}
	}

	impl TryFrom<String> for RelationalOp {
		type Error = String;

		fn try_from(value: String) -> Result<Self, Self::Error> {
			use RelationalOp::*;

			match value.as_str() {
				"<" => Ok(Lt),
				"<=" => Ok(Leq),
				">" => Ok(Gt),
				">=" => Ok(Geq),
				"==" => Ok(Eq),
				"!=" => Ok(Neq),
				_ => Err(format!("Unknown relational operator '{value}'."))
			}
		}
	}

	#[derive(Debug, Clone)]
	enum AssignmentOp {
		Assign,
	}

	impl Display for AssignmentOp {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			write!(f, ":=")
		}
	}

	impl TryFrom<String> for AssignmentOp {
		type Error = String;

		fn try_from(value: String) -> Result<Self, Self::Error> {
			if value == ":=" {
				Ok(AssignmentOp::Assign)
			} else {
				Err(format!("Unknown assignment operator '{value}'."))
			}
		}
	}

	#[derive(Debug)]
	enum Operator {
		ArithmeticOp(ArithmeticOp),
		AssignmentOp(AssignmentOp),
		BinaryOp(BinaryOp),
		RelationalOp(RelationalOp),
	}

	impl Display for Operator {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use Operator::*;

			match &*self {
				ArithmeticOp(op) => Ok(op.fmt(f).unwrap()),
				AssignmentOp(op) => Ok(op.fmt(f).unwrap()),
				BinaryOp(op) => Ok(op.fmt(f).unwrap()),
				RelationalOp(op) => Ok(op.fmt(f).unwrap()),
			}
		}
	}

	impl TryFrom<String> for Operator {
		type Error = String;

		fn try_from(value: String) -> Result<Self, Self::Error> {
			if value ==":=" {
				Ok(Operator::AssignmentOp(AssignmentOp::Assign))
			} else if let Ok(op) = ArithmeticOp::try_from(value.clone()) {
				Ok(Operator::ArithmeticOp(op))
			} else if let Ok(op) = BinaryOp::try_from(value.clone()) {
				Ok(Operator::BinaryOp(op))
			} else if let Ok(op) = RelationalOp::try_from(value.clone()) {
				Ok(Operator::RelationalOp(op))
			} else {
				Err(format!("Unknown operator '{value}'."))
			}
		}
	}
}

pub mod decl {
	use crate::lexer::{keyword::Type, literal::IntegerLiteral};
	use std::fmt::{self, Display, Formatter};

	/// Size 2..n
	#[derive(Debug, Clone)]
	pub enum Declaration {
		Var(Type, String),
		Array(Type, Vec<IntegerLiteral>, String),
		Record(Vec<Declaration>, String),
	}

	impl Display for Declaration {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use Declaration::*;

			match &*self {
				Var(_type, id) => write!(f, "{} {};", _type, id),
				Array(_type, sizes, id) => write!(f, "[{}; {:?}] {};", _type, sizes, id),
				Record(decls, id) => write!(f, "{{{:?}}} {};", decls, id),
			}
		}
	}
}

pub mod expr {
	use std::{convert::TryFrom, fmt::{self, Display, Formatter}, string::String};

	/// Size 1
	#[derive(Debug, Clone)]
	pub enum ArithmeticLiteral {
		Int(isize),
		Float(f64),
	}

	impl Display for ArithmeticLiteral {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use ArithmeticLiteral::*;

			match *self {
				Int(int) => write!(f, "{int}"),
				Float(float) => write!(f, "{float}"),
			}
		}
	}

	impl TryFrom<String> for ArithmeticLiteral {
		type Error = String;

		fn try_from(value: String) -> Result<Self, Self::Error> {
			use ArithmeticLiteral::*;

			if let Ok(int) = value.parse::<isize>() {
				Ok(Int(int))
			} else if let Ok(float) = value.parse::<f64>() {
				Ok(Float(float))
			} else {
				Err(format!("Unknown arithmetic literal '{value}'."))
			}
		}
	}

	/// Size 1..n
	#[derive(Debug, Clone)]
	pub enum LvalueExpr {
		Variable(String),
		ArrayIndex(String, Box<ArithmeticOperation>),
		RecordMember(String, String),
	}

	impl Display for LvalueExpr {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use LvalueExpr::*;

			match &*self {
				Variable(id) => write!(f, "{id}"),
				ArrayIndex(id, op) => write!(f, "{id}, {:?}", op),
				RecordMember(id, mem_id) => write!(f, "{id}, {mem_id}"),
			}
		}
	}

	impl TryFrom<Vec<String>> for LvalueExpr {
		type Error = String;

		fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
			use LvalueExpr::*;

			match value.len() {
				0 => Err("At least one element is required for a lvalue expression.".to_string()),
				1 => Ok(Variable(value[0].clone())),
				2 => Ok(RecordMember(value[0].clone(), value[1].clone())),
				_ => Err(format!("Unknown lvalue expression '{:?}'.", value)),
			}
		}
	}

	/// Size 3..n
	pub type ArithmeticOperation = (ArithmeticExpr, super::ops::ArithmeticOp, ArithmeticExpr);
	pub type RelationalOperation = (ArithmeticExpr, super::ops::RelationalOp, ArithmeticExpr);
	pub type BinaryOperation = (BooleanExpr, super::ops::BinaryOp, BooleanExpr);

	/// Size 1..n
	#[derive(Debug, Clone)]
	pub enum ArithmeticExpr {
		Literal(ArithmeticLiteral),
		LvalueExpr(LvalueExpr),
		ArithmeticOperation(Box<ArithmeticOperation>),
	}

	impl Display for ArithmeticExpr {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use ArithmeticExpr::*;

			match &*self {
				Literal(literal) => write!(f, "{literal}"),
				LvalueExpr(lvalue) => write!(f, "{lvalue}"),
				ArithmeticOperation(op) => write!(f, "{:?}", op),
			}
		}
	}

	/// Size 1..n
	#[derive(Debug, Clone)]
	pub enum BooleanExpr {
		BooleanLiteral(bool),
		NotOperation(Box<BooleanExpr>),
		RelationalOperation(ArithmeticExpr, super::ops::RelationalOp, ArithmeticExpr),
		BinaryOperation(Box<BooleanExpr>, super::ops::BinaryOp, Box<BooleanExpr>),
	}

	impl Display for BooleanExpr {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use BooleanExpr::*;

			match &*self {
				BooleanLiteral(boolean) => write!(f, "{boolean}"),
				NotOperation(boolex) => write!(f, "-{boolex}"),
				RelationalOperation(arex1, relop, arex2) => write!(f, "{arex1} {relop} {arex2}"),
				BinaryOperation(boolex1, binop, boolex2) => write!(f, "{boolex1} {binop} {boolex2}"),
			}
		}
	}

	/// 3..n
	#[derive(Debug)]
	pub enum Expression {
		ArithmeticExpr(ArithmeticExpr),
		BooleanExpr(BooleanExpr),
		LvalueExpr(LvalueExpr),
	}

	impl Display for Expression {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use Expression::*;

			match &*self {
				ArithmeticExpr(expr) => write!(f, "{expr}"),
				BooleanExpr(expr) => write!(f, "{expr}"),
				LvalueExpr(expr) => write!(f, "{expr}"),
			}
		}
	}
}

pub mod stmt {
	use std::fmt::{self, Display, Formatter};
	use super::{expr::{ArithmeticExpr, BooleanExpr, LvalueExpr}, decl::Declaration};

	pub type Scope = (Vec<Declaration>, Vec<Statement>);

	/// Size 0..n
	#[derive(Debug, Clone)]
	pub enum Statement {
		LvalueAssign(LvalueExpr, ArithmeticExpr),
		RecordAssign(String, Vec<ArithmeticExpr>),
		If(BooleanExpr, Box<Scope>),
		IfElse(BooleanExpr, Box<Scope>, Box<Scope>),
		While(BooleanExpr, Box<Scope>),
		Read(LvalueExpr),
		Write(ArithmeticExpr),
		Break,
		Continue,
		Scope(Scope),
	}

	impl Display for Statement {
		fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
			use Statement::*;

			match &*self {
				LvalueAssign(lvalue, arex) => write!(f, "{lvalue} := {arex};"),
				RecordAssign(recid, arex) => write!(f, "{recid} := {:?};", arex),
				If(boolex, scope) => write!(f, "if {boolex} {:?}", scope),
				IfElse(boolex, scope1, scope2) => write!(f, "if {boolex} {:?} else {:?}", scope1, scope2),
				While(boolex, scope) => write!(f, "while {boolex} {:?}", scope),
				Read(lvalue) => write!(f, "read {lvalue};"),
				Write(arex) => write!(f, "write {arex};"),
				Break => write!(f, "break;"),
				Continue => write!(f, "continue;"),
				Scope((decls, stmts)) => if decls.is_empty() && stmts.is_empty() {
					write!(f, ";")
				} else if decls.len() + stmts.len() == 1 {
					write!(f, "{:?} {:?}", decls, stmts)
				} else {
					write!(f, "{{\n{:?}\n{:?}\n}}", decls, stmts)
				}
			}
		}
	}
}
