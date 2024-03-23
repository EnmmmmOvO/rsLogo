#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
	NUM(i32, usize, usize),
	FLOAT(f32, usize, usize),
	VAR(String, usize, usize),
	ADD(Box<Expr>, Box<Expr>, usize, usize),
	SUB(Box<Expr>, Box<Expr>, usize, usize),
	MUL(Box<Expr>, Box<Expr>, usize, usize),
	DIV(Box<Expr>, Box<Expr>, usize, usize),
	EQ(Box<Expr>, Box<Expr>, usize, usize),
	NE(Box<Expr>, Box<Expr>, usize, usize),
	LT(Box<Expr>, Box<Expr>, usize, usize),
	GT(Box<Expr>, Box<Expr>, usize, usize),
	AND(Box<Expr>, Box<Expr>, usize, usize),
	OR(Box<Expr>, Box<Expr>, usize, usize),
	XCOR(usize, usize),
	YCOR(usize, usize),
	HEADING(usize, usize),
	COLOR(usize, usize),
	ERROR(String, usize, usize),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
	IF(Box<Expr>, Vec<Stmt>, usize, usize),
	WHILE(Box<Expr>, Vec<Stmt>, usize, usize),
	MAKE(Box<Assign>, Box<Expr>, usize, usize),
	PENUP(usize, usize),
	PENDOWN(usize, usize),
	FORWARD(Box<Expr>, usize, usize),
	BACK(Box<Expr>, usize, usize),
	LEFT(Box<Expr>, usize, usize),
	RIGHT(Box<Expr>, usize, usize),
	SETPENCOLOR(Box<Expr>, usize, usize),
	TURN(Box<Expr>, usize, usize),
	SETHEADING(Box<Expr>, usize, usize),
	SETX(Box<Expr>, usize, usize),
	SETY(Box<Expr>, usize, usize),
	ADDASSIGN(Box<Assign>, Box<Expr>, usize, usize),
	FUNC1(Box<DeclName>, Box<Expr>, usize, usize),
	FUNC0(Box<DeclName>, usize, usize),
	COMMENT(String),
}

#[derive(Debug, PartialEq)]
pub enum Decl {
	FUNC1(Box<DeclName>, Box<Assign>, Vec<Stmt>),
	FUNC0(Box<DeclName>, Vec<Stmt>),
}

#[derive(Debug, PartialEq)]
pub enum Assign {
	VAR(String, usize, usize),
	ERROR(String, usize, usize),
}

#[derive(Debug, PartialEq)]
pub enum DeclName {
	STRING(String, usize, usize),
	ERROR(String, usize, usize),
}