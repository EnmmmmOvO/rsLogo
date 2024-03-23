use std::collections::HashMap;

pub enum VariableType {
	Float(f32),
	Int(i32),
	Var(String),
}

pub struct Variable {
	map: HashMap<String, VariableType>,
}

impl Variable {
	pub fn new() -> Self {
		Variable {
			map: HashMap::new(),
		}
	}

	pub fn insert(&mut self, name: String, var: VariableType) -> bool {
		if self.map.contains_key(&name) {
			return false;
		} else {
			self.map.insert(name, var);
			return true;
		}
	}

	pub fn get(&self, name: &str) -> Option<&VariableType> {
		self.map.get(name)
	}

	pub fn get_mut(&mut self, name: &str) -> Option<&mut VariableType> {
		self.map.get_mut(name)
	}
}