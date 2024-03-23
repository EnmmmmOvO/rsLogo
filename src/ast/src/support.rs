use regex::Regex;

pub fn check_name(input: &str) -> bool {
	Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap().is_match(input)
}
