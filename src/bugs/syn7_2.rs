use crate::bugs::Bug;
use std::ptr;
use std::marker::PhantomData;

use std::fmt::Debug;

pub struct Handle<'a, S: ?Sized> {
	value: *const S,
	phantom: PhantomData<&'a S>
}

// Bug
pub fn external<'a, S: ?Sized + 'a, T: AsMut<S>>(mut data: T) -> Handle<'a, S> {

	let buffer: &S = data.as_mut();
	Handle{value: buffer as *const S, phantom: PhantomData}
}

impl<'a, S: ?Sized> Handle<'a, S> {

	pub fn get_data(&mut self) -> &S {
		unsafe{&*(self.value)}
	}
}


pub struct Syn7_2;

impl Bug for Syn7_2 {

	fn exploit() {

		let mut data: Vec<u8> = Vec::from([1u8, 2, 3]);

		let mut handle: Handle<'_, [u8]> = external(data.as_mut_slice());

		drop(data);

		let ret: &[u8] = handle.get_data();

		println!("{:?}", ret);
	}
}
