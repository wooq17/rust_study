use std::collections::HashMap;
use std::borrow::BorrowMut;

use super::nodes::node::{Node, NodeEnum, Handle};

pub struct JsonObject {
	root: Handle,
}

enum ParsingState {
	Key,
	Value,
	None,
}

enum StringState {
	Parsing,
	None,
}

fn parse(json_string: &str) -> Handle {
	let mut stack: Vec<Handle> = vec!();
	
	let mut parsing_state = ParsingState::Value;
	let mut string_state = StringState::None;
	let mut current_string = String::new();
	let mut current_key = String::new();
	let mut current_node = Node::new_node("sentinel", NodeEnum::None);
	
	for each_char in json_string.chars() {
		match string_state {
			StringState::Parsing => { 
				match each_char {
					'"' => { 
						string_state = StringState::None;
						match parsing_state {
							ParsingState::Value => { 
								println!("key : {} / value : {}", &current_key, &current_string);
								
								let new_node = Node::new_node(&current_key, NodeEnum::Text(current_string));
								current_node.add_child(&current_key, new_node);
							
								current_string = String::new();
								parsing_state = ParsingState::None;
							},
							_ => {},
						}
					},
					_ => { current_string.borrow_mut().push(each_char); }, 
				}
			},
			StringState::None => {
				match each_char {
					'{' => { 
						match parsing_state {
							ParsingState::Value => { 
								stack.push(current_node);
								current_node = Node::new_node(&current_key, NodeEnum::Composite(HashMap::<String, Handle>::new()));
								
								parsing_state = ParsingState::Key; 
							},
							_ => {},
						}
					},
					'}' => {
						if stack.len() == 1 { continue; }
					
						match stack.pop() {
							Some(mut handle) => { 
								handle.add_child(&current_node.get_key(), current_node);
								current_node = handle; 
							},
							_ => panic!("invalid JSON format."),
						}
					},
					'"' => { string_state = StringState::Parsing; },
					':' => { 
						current_key = current_string.clone();
						current_string = String::new();
						
						parsing_state = ParsingState::Value;
					},
					',' => { parsing_state = ParsingState::Key; }, // need to validate input
					' ' | '\t' | '\n' | '\r' => {},
					_ => panic!("invalid JSON format."),
				}
			},
		}
	}

	// for debug
	println!("root key : {}", current_node.get_key());

	current_node
}

impl JsonObject {
	pub fn new(json_string: &str) -> JsonObject {
		JsonObject { root: parse(json_string) }
	}
	
	pub fn get_root(&self) -> Handle {
		self.root.clone()
	}
}