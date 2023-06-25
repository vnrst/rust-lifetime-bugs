use crate::bugs::Bug;
use std::ptr;
use std::marker::PhantomData;

use std::fmt::Debug;

pub struct Handle<'a> {
	value: *const [u8],
	phantom: PhantomData<&'a u8>
}

// Bug
pub fn external<'a, T: AsMut<[u8]>>(mut data: T) -> Handle<'a> {

	let buffer: &[u8] = data.as_mut();
	Handle{value: buffer as *const [u8], phantom: PhantomData}
}

impl<'a> Handle<'a> {

	pub fn get_data(&mut self) -> &[u8] {
		unsafe{&*(self.value)}
	}
}

pub struct Syn7_1;

impl Bug for Syn7_1 {

	fn exploit() {

		let mut data: Vec<u8> = Vec::from([1u8, 2, 3]);

		let mut handle: Handle = external(data.as_mut_slice());

		drop(data);

		let ret: &[u8] = handle.get_data();

		println!("{:?}", ret);
	}
}
