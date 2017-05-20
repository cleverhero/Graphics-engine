extern crate glium;

use std::collections::HashMap;
use texture::CTexture;
use glium::Program;
use std::rc::Rc;

pub struct Pool<T> {
	elements: HashMap<&'static str, T>,
}

impl<T> Pool<T> {
	pub fn new() -> Pool<T> {
		Pool{ elements: HashMap::new() }
	}

	pub fn insert(&mut self, key: &'static str, value: T) {
		self.elements.insert(key, value);
	}

	pub fn remove(&mut self, key: &'static str) {
		self.elements.remove(key);
	}

	pub fn get_element(&mut self, key: &'static str) -> &T {
		self.elements.get(key).unwrap()
	}
}