use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Type {
	B(bool),
	F(f32),
}

#[derive(Debug, PartialEq)]
pub struct Variable {
	map: HashMap<String, Option<Type>>,
}

impl Variable {
	pub fn new() -> Self {
		Variable { map: HashMap::new(), }
	}

	pub fn insert_num(&mut self, name: String, var: Option<f32>) {
		match var {
			Some(num) => self.map.insert(name, Some(Type::F(num))),
			None => self.map.insert(name, None),
		};
	}

	pub fn insert_bool(&mut self, name: String, var: Option<bool>) {
		match var {
			Some(bool) => self.map.insert(name, Some(Type::B(bool))),
			None => self.map.insert(name, None),
		};
	}

	pub fn get(&self, name: &str) -> Option<&Option<Type>> {
		self.map.get(name)
	}
}