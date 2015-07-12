use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;
use std::collections::HashMap;

pub enum NodeEnum {
    /// a text node
    Text(String),
    
    /// a number
    Number(i32),
    
    /// a composite node
    Composite(HashMap<String, Handle>),
    
    /// nothing
    None,
}

#[derive(Clone)]
pub struct Handle(Rc<RefCell<Node>>);

impl Deref for Handle {
    type Target = Rc<RefCell<Node>>;
    fn deref(&self) -> &Rc<RefCell<Node>> { &self.0 }
}

impl Handle {
	pub fn add_child(&mut self, key: &str, child: Handle) {
		self.borrow_mut().add_child(key, child);
	}
	
	pub fn get_child(&self, key: &str) -> Handle {
		match self.borrow().get_child(key) {
			Some(child) => child,
			_ => panic!("no child"),
		}
	}
	
	pub fn get_string(&self) -> String {
		match self.borrow().get_string() {
			Some(value) => value,
			_ => panic!("no value"),
		}
	}
	
	pub fn get_key(&self) -> String {
		self.borrow().key.clone()
	}
}

pub struct Node {
	pub key: String,
	value: NodeEnum,
}

impl Node {
	fn new(key: &str, value: NodeEnum) -> Node {
		Node{ 
			key: key.to_string(),
			value: value,
		}
	}
	
	pub fn new_node(key: &str, node: NodeEnum) -> Handle {
    	Handle(Rc::new(RefCell::new(Node::new(key, node))))
	}
	
	pub fn get_child(&self, key: &str) -> Option<Handle> {
		match self.value {
			NodeEnum::Composite(ref children) => Some((*children.get(key).unwrap()).clone()),
			_ => None,
		}
	}
	
	pub fn get_string(&self) -> Option<String> {
		match self.value {
			NodeEnum::Text(ref value) => Some(value.clone()),
			_ => None,
		}
	}
	
	pub fn get_number(&self) -> Option<&i32> {
		match self.value {
			NodeEnum::Number(ref value) => Some(&value),
			_ => None,
		}
	}
	
	pub fn set_value(&mut self, value: NodeEnum) {
		self.value = value;
	}
	
	pub fn add_child(&mut self, key: &str, child: Handle) {
		match self.value {
			NodeEnum::Composite(ref mut map) => { map.insert(key.to_string(), child); },
			_ => panic!("only Composite can assign child node"),
		}
	}
}