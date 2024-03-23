use std::collections::HashMap;
use ast::structs::Stmt;

struct FunctionType {
	var: String,
	stmt_list: Vec<Stmt>,
}

struct Function {
	functions_map: HashMap<String, FunctionType>,
}

impl Function {
	pub fn new() -> Self {
		Function {
			functions_map: HashMap::new(),
		}
	}

	pub fn insert(&mut self, name: String, var: String, stmt_list: Vec<Stmt>) -> bool {
		if self.functions_map.contains_key(&name) {
			return false;
		} else {
			self.functions_map.insert(name, FunctionType {
				var,
				stmt_list,
			});
			return true;
		}
	}

	pub fn get_stmt_list(&self, name: &str) -> Option<&FunctionType> {
		self.functions_map.get(name)
	}

	pub fn get_mut(&mut self, name: &str) -> Option<&mut FunctionType> {
		self.functions_map.get_mut(name)
	}
}