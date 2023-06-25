use crate::bugs::Bug;

// https://rustsec.org/advisories/RUSTSEC-2022-0028.html

use std::ptr;
use std::marker::PhantomData;

use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct JsBuffer {
	buffer: *const [u8]
}

pub trait Managed: Copy {

	fn into_inner(self) -> *const [u8];
}

impl Managed for JsBuffer {

	fn into_inner(self) -> *const [u8] {
		self.buffer
	}
}

pub struct Handle<'a, J: Managed + 'a> {
	value: J,
	phantom: PhantomData<&'a J>
}

impl JsBuffer {

	// Bug
	pub fn external<'a, T: AsMut<[u8]>>(mut data: T) -> Handle<'a, Self> {

		let buffer: &[u8] = data.as_mut();
		let jsbuf = JsBuffer{buffer: buffer as *const [u8]};
		Handle{value: jsbuf, phantom: PhantomData}
	}
}

impl<'a, J: Managed + 'a> Handle<'a, J> {

	pub fn get_data(&mut self) -> &[u8] {
		unsafe{&*(self.value.into_inner())}
	}
}


pub struct Syn7;

impl Bug for Syn7 {

	fn exploit() {

		let mut data: Vec<u8> = Vec::from([1u8, 2, 3]);

		let mut handle: Handle<'_, JsBuffer> = JsBuffer::external(data.as_mut_slice());

		drop(data);

		let ret: &[u8] = handle.get_data();

		println!("{:?}", ret);
	}
}
