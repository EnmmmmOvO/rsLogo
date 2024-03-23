use std::collections::HashMap;
use ast::structs::{Assign, Expr, Stmt};


#[derive(Debug, PartialEq)]
pub struct FunctionType {
	args: Vec<Box<Assign>>,
	stmt_list: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Function {
	map: HashMap<String, FunctionType>,
}

impl Function {
	pub fn new() -> Self {
		Function { map: HashMap::new(),  }
	}

	pub fn insert(&mut self, name: String, args: Vec<Box<Assign>>, stmt_list: Vec<Stmt>) -> bool {
		if self.map.contains_key(&name) {
			return false;
		} else {
			self.map.insert(name, FunctionType { args, stmt_list });
			return true;
		}
	}

	pub fn get(&self, name: &str) -> Option<&FunctionType> {
		self.map.get(name)
	}
}