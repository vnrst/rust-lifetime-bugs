use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2021-0128.html

use std::ptr;
use std::borrow::Borrow;
use std::fmt::Debug;

pub struct MockConnection<T> {
	hook: *const T
}

impl<T: Debug> MockConnection<T> {

	pub fn new() -> Self {
		let hook = "".to_string();
		MockConnection{hook: ptr::null()}
	}
	// Bug
	// S ~~ &T
	pub fn update_hook<'c, S: 'c + Borrow<T>>(&'c mut self, hook: S) {
		self.hook = hook.borrow();
	}

	pub fn call_hook(&self) {
		unsafe{println!("{:?}", *(self.hook))};
	}
}

pub struct Syn4_2;

impl Bug for Syn4_2 {

	fn exploit() {

		let mut conn = MockConnection::<String>::new();
		let data: String = "Hello".to_string();
		conn.update_hook(&data);
		drop(data);
		conn.call_hook();
	}
}
