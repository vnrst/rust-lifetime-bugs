use crate::bugs::Bug;
use std::ptr;

// https://rustsec.org/advisories/RUSTSEC-2021-0128.html

// Replacing the closure with a simple String
pub struct MockConnection {
	hook: *const String
}

impl MockConnection {

	pub fn new() -> Self {
		let hook = "".to_string();
		MockConnection{hook: ptr::null()}
	}
	// Bug
	// Given - self.hook has to live for at least as long as 'c
	// Given - hook lives for at least as long as 'c

	// Let's say something lives for 10 years
	// I give it to someone and tell them that it lives for at least 1 year
	// They replace it with something that lives for 1 year and give it back
	// Now it's a problem, because it actually lives for 10 years

	pub fn update_hook<'c>(&'c mut self, hook: &'c String) {
		self.hook = hook;
	}

	pub fn call_hook(&self) {
		unsafe{println!("{}", *(self.hook))};
	}
}

pub struct Syn4_1;

impl Bug for Syn4_1 {

	fn exploit() {

		// Outer lifetime
		let mut conn = MockConnection::new();
		{// lifetime 'c
			let data: String = "Hello".to_string();
			conn.update_hook(&data);
		}
		conn.call_hook();
	}
}
