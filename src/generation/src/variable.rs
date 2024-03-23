use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Variable {
	map: HashMap<String, Option<f32>>,
}

impl Variable {
	pub fn new() -> Self {
		Variable { map: HashMap::new(), }
	}

	pub fn insert(&mut self, name: String, var: Option<f32>) {
		self.map.insert(name, var);
	}

	pub fn get(&self, name: &str) -> Option<&Option<f32>> {
		self.map.get(name)
	}
}