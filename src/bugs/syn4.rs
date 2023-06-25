use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2021-0128.html

use std::ptr;
use std::ffi::c_void;

pub struct Connection {
	hook: *mut c_void
}

impl Connection {

	pub fn new() -> Self {
		let hook = ||{};
		let boxed: Box<dyn Fn()> = Box::new(hook);
		// *mut dyn Fn() is a "fat" pointer (pointer to Trait obj), and *mut c_void is a "thin" pointer
		// So although we can cast from *mut dyn Fn() to *mut c_void, we can't do the reverse
		// Instead, we have a boxed boxed closure (double box).
		// *mut Box<dyn Fn()> is a thin pointer.

		Connection{hook: Box::into_raw(Box::new(boxed)) as *mut Box<dyn Fn()> as *mut c_void}
	}

	// Bug
	pub fn update_hook<'c, F>(&'c mut self, hook: F)
		where F: Fn() + 'c {
		// where F: Fn() + 'static { // Try this also

		let boxed: Box<dyn Fn()> = Box::new(hook);
		self.hook = Box::into_raw(Box::new(boxed)) as *mut Box<dyn Fn()> as *mut c_void;
	}

	pub fn call_hook(&self)
	{
		let boxed_hook: *mut Box<dyn Fn()> = self.hook as *mut Box<dyn Fn()>;

		unsafe{(*(*boxed_hook))()};
	}
}

pub struct Syn4;

impl Bug for Syn4 {

	fn exploit() {

		let mut conn = Connection::new();
		let data = "Hello".to_string();

		let closure = || {
			println!("{}", data);
		};
		conn.update_hook(closure);
		drop(data);
		conn.call_hook();
	}
}
