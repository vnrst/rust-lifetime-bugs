use crate::bugs::Bug;
use std::fmt::Debug;
use std::marker::PhantomData;

use std::ffi::c_void;

pub struct Waker<'a> {
	data: *const c_void,
	phantom: PhantomData<&'a i32>
}

// Bug
pub fn waker<'a, W>(wake: W) -> Waker<'a> {
	let data: *mut W = Box::into_raw(Box::new(wake));
	let raw = data as *mut _ as *const c_void;
	Waker{data: raw, phantom: PhantomData}
}

impl<'a> Waker<'a> {

	pub fn get_data<T>(&mut self) -> &T {
		unsafe{&*(self.data as *const T)}
	}
}

pub struct Syn3_2;

impl Bug for Syn3_2 {

	fn exploit() {

		let mut w: Waker;

		let mut data : Vec<i32> = Vec::from([1, 2, 3]);
		w = waker(&data);

		drop(data);

		let rec_data : &&Vec<i32> = w.get_data::<&Vec<i32>>();

		println!("{:?}", rec_data);
	}
}
